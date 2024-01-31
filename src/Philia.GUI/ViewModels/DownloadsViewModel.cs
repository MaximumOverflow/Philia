using SixLabors.ImageSharp.Metadata.Profiles.Xmp;
using SixLabors.ImageSharp.PixelFormats;
using System.Collections.ObjectModel;
using SixLabors.ImageSharp.Advanced;
using System.Collections.Generic;
using System.Threading.Channels;
using System.Threading.Tasks;
using Avalonia.Threading;
using System.Text.Json;
using System.Linq;
using System.Text;
using System.Xml;
using System.IO;
using System.Threading;
using Image = SixLabors.ImageSharp.Image;

namespace Philia.GUI.ViewModels;

public sealed partial class DownloadsViewModel : ObservableObject, IDisposable
{
	[ObservableProperty]
	private int _queuedCount;
	public ObservableCollection<EntryGroup> Groups { get; } = [];
	private readonly Channel<Entry> _downloadRequestQueue;

	public DownloadsViewModel()
	{
		_downloadRequestQueue = Channel.CreateUnbounded<Entry>();
		for (var i = 0; i < Environment.ProcessorCount; i++)
		{
			var thread = new Thread(DownloadThread);
			thread.Start();
		}
	}
	
	public void EnqueueGroup(EntryGroup group)
	{
		foreach (var entry in group.Entries) _downloadRequestQueue.Writer.TryWrite(entry);
		QueuedCount += group.Entries.Count;
		Groups.Add(group);
	}

	private void DownloadThread()
	{
		while (!_downloadRequestQueue.Reader.Completion.IsCompleted)
		{
			if (!_downloadRequestQueue.Reader.TryRead(out var entry))
			{
				Thread.Sleep(500);
				continue;
			}
			Dispatcher.UIThread.Invoke(() =>
			{
				entry.State = EntryState.Downloading;
			}, DispatcherPriority.Background);

			try
			{
				Console.WriteLine($"Downloading {entry.Url}...");
				using var networkStream = App.HttpClient.GetStreamAsync(entry.Url).Result;

				var image = Image.Load<Rgba32>(networkStream);
				var metadata = new XmlDocument();
				var postInfo = metadata.CreateElement("PostInfo");
				postInfo.InnerText = JsonSerializer.Serialize(entry.Post, Source.JsonSerializerOptions);
				image.Metadata.XmpProfile = new XmpProfile(Encoding.Default.GetBytes(metadata.OuterXml));

				using var fileStream = File.OpenWrite(entry.Path);
				image.Save(fileStream, image.DetectEncoder(entry.Path));

				Dispatcher.UIThread.Invoke(() =>
				{
					entry.Group.IntProgress++;
					QueuedCount--;
					entry.State = EntryState.Downloaded;
				}, DispatcherPriority.Background);
				Console.WriteLine($"{entry.Url} downloaded successfully");
			}
			catch (Exception e)
			{
				Console.Error.WriteLine(e);
				Dispatcher.UIThread.Invoke(() =>
				{
					entry.Group.IntProgress++;
					QueuedCount--;
					return entry.State = EntryState.Failed;
				}, DispatcherPriority.Background);
			}
		}
	}
	
	public enum EntryState
	{
		Queued,
		Downloading,
		Downloaded,
		Failed,
	}

	public sealed partial class Entry : ObservableObject
	{
		public required Post Post { get; init; }
		public required string Url { get; init; }
		public required string Path { get; init; }
		public required EntryGroup Group { get; init; }
		[ObservableProperty] private EntryState _state;
	}
	
	public sealed partial class EntryGroup : ObservableObject
	{
		public Source? Source { get; }
		public DateTime DateTime { get; } = DateTime.Now;
		public IReadOnlyList<Entry> Entries { get; }
		
		[ObservableProperty]
		[NotifyPropertyChangedFor(nameof(Progress))]
		private int _intProgress;
		
		public float Progress => IntProgress / (float) Entries.Count * 100f;
		
		public EntryGroup(Source? source, IEnumerable<Post> posts)
		{
			var entries = new List<Entry>();
			foreach (var post in posts)
			{
				if(post.Media.FirstOrDefault(m => m.Original) is not {Url: {} url, Type: MediaType.Image}) 
					continue;
				
				var path = $"Downloads/{Path.GetFileName(url.AsSpan())}";
				if(Path.Exists(path)) continue;
				
				entries.Add(new Entry { Post = post, Url = url, Path = path, Group = this });
			}
			
			Source = source;
			Entries = entries;
		}

		public override string ToString()
		{
			return $"{Source?.Name ?? "Download"} - {DateTime}";
		}
	}

	public void Dispose()
	{
		_downloadRequestQueue.Writer.Complete();
	}
}