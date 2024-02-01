using System.IO;
using AsyncImageLoader.Loaders;
using Avalonia.Media.Imaging;
using System.Threading.Tasks;
using System.Net.Http;

namespace Philia.GUI.Components;

public partial class ImageGrid : UserControl
{
	public static readonly ThumbnailLoader ThumbnailLoader = new(App.HttpClient, false);
	public static readonly RamCachedWebImageLoader ImageLoader = new(App.HttpClient, false);
	
	public static readonly StyledProperty<BaseWebImageLoader> LoaderProperty =
		AvaloniaProperty.Register<ImageGridImage, BaseWebImageLoader>(nameof(Loader), ImageLoader);

	public BaseWebImageLoader Loader
	{
		get => GetValue(LoaderProperty);
		set => SetValue(LoaderProperty, value);
	}
	
	public ImageGrid()
	{
		InitializeComponent();
	}
}

public sealed class ThumbnailLoader : BaseWebImageLoader
{
	public ThumbnailLoader() {}

	public ThumbnailLoader(HttpClient httpClient, bool disposeHttpClient)
		: base(httpClient, disposeHttpClient) {}

	public override async Task<Bitmap?> ProvideImageAsync(string url)
	{
		try
		{
			await using var stream = File.OpenRead(url);
			return Bitmap.DecodeToWidth(stream, 128);
		}
		catch (Exception e)
		{
			Console.Error.WriteLine(e);
			return null;
		}
	}
}
