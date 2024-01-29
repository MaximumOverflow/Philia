using System.Collections.Generic;

namespace Philia.GUI.ViewModels;

public sealed partial class ImageSet
{
	public required IReadOnlyList<Post> Posts { get; init; }
}

public interface IImageSetView
{
	public ImageSet ImageSet { get; set; }
}
