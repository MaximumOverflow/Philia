using Avalonia.Input;

namespace Philia.GUI.Components;

public partial class ImageGridImage : UserControl
{
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