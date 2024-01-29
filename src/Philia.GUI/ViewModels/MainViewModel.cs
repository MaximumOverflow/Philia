using System.Collections.ObjectModel;
using System.Collections.Generic;
using System.Linq;

namespace Philia.GUI.ViewModels;

public sealed partial class MainViewModel : ViewModelBase
{
	public ObservableCollection<Source> Sources { get; private set; }
	public ObservableCollection<Plugin> Plugins { get; }
	public SearchViewModel Search { get; }

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

		Search = new SearchViewModel();
	}
}