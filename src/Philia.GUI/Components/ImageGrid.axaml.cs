using System.Collections.Concurrent;
using AsyncImageLoader.Loaders;
using Avalonia.Data.Converters;
using Avalonia.Media.Imaging;
using System.Threading.Tasks;
using System.Globalization;
using System.Threading;
using System.Net.Http;
using System.Linq;
using System.IO;

namespace Philia.GUI.Components;

public partial class ImageGrid : UserControl
{
	public static readonly ThumbnailLoader ThumbnailLoader = new(App.HttpClient, false);
	public static readonly RamCachedImageLoader ImageLoader = new(App.HttpClient, false);
	public static readonly UncachedImageLoader UncachedImageLoader = new(App.HttpClient, false);
	
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


public class UncachedImageLoader(HttpClient httpClient, bool disposeHttpClient) : BaseWebImageLoader(httpClient, disposeHttpClient)
{
	private readonly SemaphoreSlim _semaphore = new(3, 3);
	public override async Task<Bitmap?> ProvideImageAsync(string url)
	{
		await _semaphore.WaitAsync();
		var bitmap = await LoadAsync(url).ConfigureAwait(false);
		_semaphore.Release();
		return bitmap;
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
		if (bitmap == null) _memoryCache.TryRemove(url, out _);
		_semaphore.Release();
		return bitmap;
	}

	public void ClearAndDisposeCache()
	{
		foreach (var task in _memoryCache.Values)
			if(task.IsCompleted) 
				task.Result?.Dispose();
		
		_memoryCache.Clear();
		GC.Collect(0);
	}
}

public sealed class ThumbnailLoader(HttpClient httpClient, bool disposeHttpClient) : BaseWebImageLoader(httpClient, disposeHttpClient)
{
	private readonly SemaphoreSlim _semaphore = new(3, 3);
	public override async Task<Bitmap?> ProvideImageAsync(string url)
	{
		try
		{
			await _semaphore.WaitAsync();
			await using var stream = File.OpenRead(url);
			return Bitmap.DecodeToWidth(stream, 192);
		}
		catch (Exception e)
		{
			Console.Error.WriteLine(e);
			return null;
		}
		finally
		{
			_semaphore.Release();
		}
	}
}

public sealed class PostToImageConverter : IValueConverter
{
	public static readonly PostToImageConverter Instance = new();
	public object? Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
	{
		if (value is not Post post || targetType != typeof(string)) return null;
		return post.Media.FirstOrDefault(m => m.Original).Url;
	}

	public object? ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
	{
		throw new NotSupportedException();
	}
}
