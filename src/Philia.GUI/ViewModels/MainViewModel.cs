using System.Collections.ObjectModel;
using System.Collections.Generic;
using System.Linq;

namespace Philia.GUI.ViewModels;

public sealed partial class MainViewModel : ViewModelBase, IDisposable
{
	public SearchViewModel Search { get; }
	public GalleryViewModel Gallery { get; }
	public DownloadsViewModel Downloads { get; }
	public ObservableCollection<Plugin> Plugins { get; }
	public ObservableCollection<Source> Sources { get; private set; }

	public MainViewModel(IEnumerable<Plugin> plugins)
	{
		Plugins = new ObservableCollection<Plugin>(plugins);
		Sources = new ObservableCollection<Source>(Plugins.SelectMany(p => p.Sources));
		Plugins.CollectionChanged += (_, e) =>
		{
			Sources.Clear();
			foreach (var source in Plugins.SelectMany(p => p.Sources))
				Sources.Add(source);
		};

		Search = new SearchViewModel(Sources);
		Downloads = new DownloadsViewModel();
		Gallery = new GalleryViewModel();
	}

	public void Dispose()
	{
		Downloads.Dispose();
	}
}