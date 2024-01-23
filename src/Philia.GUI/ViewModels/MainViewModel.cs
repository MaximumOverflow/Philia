using CommunityToolkit.Mvvm.Input;

namespace Philia.GUI.ViewModels;

public enum CurrentTab
{
	Search,
	Gallery,
	Datasets,
	Downloads,
	Plugins,
	Settings,
}

public sealed partial class MainViewModel : ViewModelBase
{
	[ObservableProperty]
	private CurrentTab _currentTab;

	[RelayCommand]
	private void SetCurrentTab(CurrentTab tab)
	{
		CurrentTab = tab;
	}
}