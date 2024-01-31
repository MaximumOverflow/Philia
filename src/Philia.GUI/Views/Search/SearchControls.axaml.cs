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
		if(DataContext is not MainViewModel {Search: var search}) return;
		SearchSBB.Instance.Search(search, search.Query).ConfigureAwait(false);
	}

	private void Download(object? sender, RoutedEventArgs e)
	{
		if(DataContext is not MainViewModel {Search: var search, Downloads: var downloads})
			return;

		var group = new DownloadsViewModel.EntryGroup(search.Source, search.ImageSet.Posts);
		downloads.EnqueueGroup(group);
	}
}