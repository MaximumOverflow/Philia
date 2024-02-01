using System.Collections.ObjectModel;
using System.Collections.Generic;
using System.Collections.Frozen;
using System.Threading.Tasks;
using Avalonia.Threading;

namespace Philia.GUI.ViewModels;

public sealed partial class ImageSet : ObservableObject
{
	[ObservableProperty]
	[NotifyPropertyChangedFor(nameof(Show))]
	private IReadOnlyList<Post>? _filtered;
	public required ObservableCollection<Post> Posts { get; init; }

	public IReadOnlyList<Post> Show => Filtered ?? Posts;

	public async Task Filter(IEnumerable<string> include, IEnumerable<string> exclude)
	{
		var included = include.ToFrozenSet();
		var excluded = exclude.ToFrozenSet();

		if (included.Count == 0 && excluded.Count == 0)
		{
			await Dispatcher.UIThread.InvokeAsync(() => Filtered = null);
			return;
		}
		
		var filtered = new List<Post>();
		foreach (var post in Posts)
		{
			if (post.Tags.Tags.Overlaps(excluded)) continue;
			if (post.Tags.Tags.Overlaps(included)) filtered.Add(post);
		}
		await Dispatcher.UIThread.InvokeAsync(() => Filtered = filtered);
	}
}

public interface IImageSetView
{
	public ImageSet ImageSet { get; set; }
}
