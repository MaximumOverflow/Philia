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
				GC.Collect(0);
				GC.Collect(1);
				GC.Collect(2);
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