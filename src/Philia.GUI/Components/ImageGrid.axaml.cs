using System.Collections.Concurrent;
using AsyncImageLoader.Loaders;
using Avalonia.Media.Imaging;
using System.Threading.Tasks;
using System.Threading;
using System.Net.Http;
using System.IO;

namespace Philia.GUI.Components;

public partial class ImageGrid : UserControl
{
	public static readonly ThumbnailLoader ThumbnailLoader = new(App.HttpClient, false);
	public static readonly RamCachedImageLoader ImageLoader = new(App.HttpClient, false);
	
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

public class RamCachedImageLoader(HttpClient httpClient, bool disposeHttpClient) : BaseWebImageLoader(httpClient, disposeHttpClient)
{
	private readonly SemaphoreSlim _semaphore = new(3, 3);
	private readonly ConcurrentDictionary<string, Task<Bitmap?>> _memoryCache = new();
	
	public override async Task<Bitmap?> ProvideImageAsync(string url)
	{
		await _semaphore.WaitAsync();
		var bitmap = await _memoryCache.GetOrAdd(url, LoadAsync).ConfigureAwait(false);
		if (bitmap == null) _memoryCache.TryRemove(url, out Task<Bitmap> _);
		_semaphore.Release();
		return bitmap;
	}

	public void ClearCache()
	{
		_memoryCache.Clear();
		GC.Collect(0);
	}
}

public sealed class ThumbnailLoader : BaseWebImageLoader
{
	public ThumbnailLoader(HttpClient httpClient, bool disposeHttpClient)
		: base(httpClient, disposeHttpClient) {}

	public override async Task<Bitmap?> ProvideImageAsync(string url)
	{
		try
		{
			await using var stream = File.OpenRead(url);
			return Bitmap.DecodeToWidth(stream, 192);
		}
		catch (Exception e)
		{
			Console.Error.WriteLine(e);
			return null;
		}
	}
}
