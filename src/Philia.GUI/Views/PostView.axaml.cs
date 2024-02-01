using Avalonia.Media.Imaging;
using Avalonia.Interactivity;
using AsyncImageLoader;

namespace Philia.GUI;

public partial class PostView : UserControl
{
	public PostView()
	{
		InitializeComponent();
	}

	protected override void OnInitialized()
	{
		ZoomBorder.AutoFit();
		base.OnInitialized();
	}

	protected override void OnUnloaded(RoutedEventArgs e)
	{
		if(Image.CurrentImage is AdvancedImage.ImageWrapper { ImageImplementation: Bitmap bitmap })
			bitmap.Dispose();
		
		base.OnUnloaded(e);
	}
}