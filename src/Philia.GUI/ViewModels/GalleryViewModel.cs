using System.Collections.ObjectModel;
using Image = SixLabors.ImageSharp.Image;
using Size = System.Drawing.Size;
using System.Text.Json;
using System.IO;

namespace Philia.GUI.ViewModels;

public sealed partial class GalleryViewModel : ObservableObject, ISearchBarContext
{
	public ImageSet ImageSet { get; }
	public ObservableCollection<string> Query { get; } = [];

	public GalleryViewModel()
	{
		ImageSet = new ImageSet { Posts = [] };
		foreach (var file in Directory.EnumerateFiles(App.DownloadDir))
			TryLoadImage(file);
	}
	
	public void TryLoadImage(string path)
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