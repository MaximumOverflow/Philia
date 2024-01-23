using System.Collections.Generic;
using System.Collections.ObjectModel;

namespace Philia.GUI.ViewModels;

public sealed partial class MainViewModel : ViewModelBase
{
	public readonly ObservableCollection<Plugin> Plugins;

	public MainViewModel(IEnumerable<Plugin> plugins)
	{
		Plugins = new ObservableCollection<Plugin>(plugins);
	}
}