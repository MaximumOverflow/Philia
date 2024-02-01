using Avalonia.Input;

namespace Philia.GUI.Views;

public partial class MainView : UserControl
{
	public MainView()
	{
		InitializeComponent();
	}

	protected override void OnKeyDown(KeyEventArgs e)
	{
		switch (e.Key)
		{
			case Key.F9:
			{
				e.Handled = true;
				GC.Collect(0, GCCollectionMode.Forced);
				GC.Collect(1, GCCollectionMode.Forced);
				GC.Collect(2, GCCollectionMode.Forced);
				return;
			}

			default:
			{
				base.OnKeyDown(e);
				return;
			}
		}
	}
}