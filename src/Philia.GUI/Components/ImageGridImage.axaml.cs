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
}