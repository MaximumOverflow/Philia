using AsyncImageLoader.Loaders;
using Avalonia.Input;
using Avalonia.Interactivity;

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
		if(DataContext is not Post post) return;
		var last = post.Source.LastIndexOf('.');
		var source = last >= 0 ? post.Source[(last + 1)..] : post.Source;
		var window = new PostWindow { Title = $"{source} - Post {post.Id}", DataContext = DataContext };
		if (TopLevel.GetTopLevel(this) is MainWindow parent) window.Show(parent);
		else window.Show();
	}
}