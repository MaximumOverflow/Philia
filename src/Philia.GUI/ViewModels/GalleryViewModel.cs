using Image = SixLabors.ImageSharp.Image;
using Size = System.Drawing.Size;
using System.Text.Json;
using System.IO;

namespace Philia.GUI.ViewModels;

public sealed partial class GalleryViewModel : ObservableObject
{
	public ImageSet ImageSet { get; }
	public FileSystemWatcher Watcher { get; }

	public GalleryViewModel()
	{
		ImageSet = new ImageSet { Posts = [] };
		Watcher = new FileSystemWatcher();
		Watcher.Path = App.DownloadDir;
		Watcher.Filter = "*.*";
		Watcher.EnableRaisingEvents = true;
		Watcher.NotifyFilter = NotifyFilters.LastWrite | NotifyFilters.Size; 
		Watcher.Created += (_, a) =>
		{
			Console.WriteLine(a.ChangeType);
			TryLoadImage(a.FullPath);
		};
		
		foreach (var file in Directory.EnumerateFiles(App.DownloadDir))
			TryLoadImage(file);
	}
	
	private void TryLoadImage(string path)
	{
		try
		{
			var image = Image.Identify(path);
			if(image.Metadata.XmpProfile?.GetDocument() is not {} document) return;
			if(document.Element("philia_metadata") is not {} metadata) return;
			var post = JsonSerializer.Deserialize<Post>(metadata.Value);
			var media = new Media[post.Media.Length + 1];
			media[0] = new Media
			{
				Url = path,
				Original = true,
				Type = MediaType.Image,
				Dimensions = new Size(image.Width, image.Height)
			};
			post.Media.AsSpan().CopyTo(media.AsSpan(1));
			ImageSet.Posts.Add(post with { Media = media });
		}
		catch (Exception e)
		{
			Console.Error.WriteLine(e);
		}
	}
}