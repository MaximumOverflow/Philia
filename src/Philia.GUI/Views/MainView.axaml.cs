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
				GC.Collect();
				return;
			}
		}
	}
}