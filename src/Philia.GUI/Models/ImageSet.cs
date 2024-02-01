using System.Collections.ObjectModel;

namespace Philia.GUI.ViewModels;

public sealed partial class ImageSet
{
	public required ObservableCollection<Post> Posts { get; init; }
}

public interface IImageSetView
{
	public ImageSet ImageSet { get; set; }
}
