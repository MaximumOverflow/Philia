using System.Collections.ObjectModel;

namespace Philia.GUI.ViewModels;

public sealed partial class SearchViewModel : ObservableObject, IImageSetView, ISearchBarContext
{
	[ObservableProperty]
	private uint _page = 1;
	
	[ObservableProperty]
	private uint _postsPerPage = 32;

	[ObservableProperty]
	private object? _source = string.Empty;
	
	[ObservableProperty]
	private PostOrder _sorting;
	
	[ObservableProperty]
	private ImageSet _imageSet = new() { Posts = Array.Empty<Post>() };

	public ObservableCollection<string> Query { get; } = [];
}