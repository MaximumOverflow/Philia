using AsyncImageLoader.Loaders;
using Avalonia.Input;

namespace Philia.GUI.Components;

public partial class ImageGridImage : UserControl
{
	public static readonly StyledProperty<BaseWebImageLoader> LoaderProperty =
		AvaloniaProperty.Register<ImageGridImage, BaseWebImageLoader>(nameof(Loader), ImageGrid.ImageLoader);

	public BaseWebImageLoader Loader
	{
		get => GetValue(LoaderProperty);
		set => SetValue(LoaderProperty, value);
	}
	
	private PostWindow? _window;
	
	public ImageGridImage()
	{
		InitializeComponent();
	}

	protected override void OnPointerPressed(PointerPressedEventArgs e)
	{
		PseudoClasses.Add("pressed");
		base.OnPointerPressed(e);
	}

	protected override void OnPointerReleased(PointerReleasedEventArgs e)
	{
		PseudoClasses.Remove("pressed");
		base.OnPointerReleased(e);
	}

	private void OpenPost(object? sender, TappedEventArgs e)
	{
		if (_window is not null) return;
		if(DataContext is not Post post) return;
		
		var last = post.Source.LastIndexOf('.');
		var source = last >= 0 ? post.Source[(last + 1)..] : post.Source;
		
		_window = new PostWindow { Title = $"{source} - Post {post.Id}", DataContext = DataContext };
		_window.Closed += (_, _) => _window = null;
		
		if (TopLevel.GetTopLevel(this) is MainWindow parent) _window.Show(parent);
		else _window.Show();
	}
}