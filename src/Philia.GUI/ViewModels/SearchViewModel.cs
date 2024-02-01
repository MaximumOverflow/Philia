using System.Collections.ObjectModel;
using Philia.GUI.Components;

namespace Philia.GUI.ViewModels;

public sealed partial class SearchViewModel : ObservableObject, IImageSetView, ISearchBarContext
{
	[ObservableProperty]
	private uint _page = 1;
	
	[ObservableProperty]
	private uint _postsPerPage = 32;

	[ObservableProperty]
	private Source? _source;
	
	[ObservableProperty]
	private PostOrder _sorting;
	
	[ObservableProperty]
	private ImageSet _imageSet = new() { Posts = [] };

	public ObservableCollection<string> Query { get; } = [];
	public RamCachedImageLoader ImageLoader { get; } = new(App.HttpClient, false);

	public SearchViewModel(ObservableCollection<Source> sources)
	{
		if (sources is [var source, ..])
			Source = source;
	}
}