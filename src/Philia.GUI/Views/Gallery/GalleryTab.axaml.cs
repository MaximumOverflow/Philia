using System.Collections.Generic;
using System.Threading.Tasks;
using System.Linq;

namespace Philia.GUI.Views;

public partial class GalleryTab : UserControl
{
	public GalleryTab()
	{
		InitializeComponent();
	}
}

public sealed class GallerySBB : ISearchBarBehaviour
{
	public static readonly GallerySBB Instance = new();
	
	public async Task Search(object? context, IReadOnlyList<string> query)
	{
		if(context is not GalleryViewModel gallery) return;
		var include = query.Where(t => !t.StartsWith('-'));
		var exclude = query.Where(t => t.StartsWith('-')).Select(s => s[1..]);
		await gallery.ImageSet.Filter(include, exclude);
	}
}