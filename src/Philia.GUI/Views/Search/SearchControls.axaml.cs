using Avalonia.Interactivity;

namespace Philia.GUI.Views;

public partial class SearchControls : UserControl
{
	public SearchControls()
	{
		InitializeComponent();
	}

	private void Search(object? sender, RoutedEventArgs e)
	{
		if(DataContext is not ISearchBarContext search) return;
		SearchSBB.Instance.Search(DataContext, search.Query).ConfigureAwait(false);
	}
}