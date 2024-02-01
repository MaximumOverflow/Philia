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
}