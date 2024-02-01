using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Threading.Tasks;
using System.Linq;
using Avalonia.Threading;

namespace Philia.GUI.Views;

public partial class SearchTab : UserControl
{
	public SearchTab()
	{
		InitializeComponent();
	}
}

public sealed class SearchSBB : ISearchBarBehaviour
{
	public static readonly SearchSBB Instance = new();
	
	public async Task Search(object? context, IReadOnlyList<string> query)
	{
		if(context is not SearchViewModel search) return;
		if(search.Source is not ISearchPosts source) return;

		var include = query.Where(t => !t.StartsWith('-'));
		var exclude = query.Where(t => t.StartsWith('-')).Select(s => s[1..]);

		var page = search.Page;
		if (page > 0) page--;
		
		try
		{
			var posts = await source.SearchPosts(page, search.PostsPerPage, search.Sorting, include, exclude);
			await Dispatcher.UIThread.InvokeAsync(() =>
			{
				search.ImageLoader.ClearCache();
				search.ImageSet = new ImageSet { Posts = new ObservableCollection<Post>(posts) };
			});
			Console.WriteLine($"Search returned {posts.Length} posts");
		}
		catch (Exception e)
		{
			Console.Error.WriteLine(e);
		}
	}
}
