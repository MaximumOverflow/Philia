using AsyncImageLoader.Loaders;
using Avalonia.Media.Imaging;
using System.Threading.Tasks;
using System.Net.Http;

namespace Philia.GUI.Components;

public partial class ImageGrid : UserControl
{
	public ImageGrid()
	{
		InitializeComponent();
	}
	
	public static readonly RamCachedWebImageLoader ImageLoader = new(App.HttpClient, false);
}

//TODO implement this
public sealed class CachedImageLoader : BaseWebImageLoader
{
	public CachedImageLoader() {}

	public CachedImageLoader(HttpClient httpClient, bool disposeHttpClient)
		: base(httpClient, disposeHttpClient) {}

	public override Task<Bitmap?> ProvideImageAsync(string url)
	{
		return base.ProvideImageAsync(url);
	}
}
