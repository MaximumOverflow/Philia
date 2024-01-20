using System.Drawing;

namespace Philia;

public record struct Post
{
	public required string Source { get; init; }

	public required ulong Id { get; init; }
	
	public required long Score { get; init; }

	public required Rating Rating { get; init; }

	public required TagCollection Tags { get; init; }

	public string? Hash { get; init; }

	public required Media[] Media { get; init; }
}

public enum Rating
{
	Unknown,
	General,
	Safe,
	Sensitive,
	Questionable,
	Explicit,
}

public enum MediaType
{
	Unknown,
	Image,
	Video,
}

public record struct Media
{
	public required string Url { get; init; }
	public required bool Original { get; init; }
	public required MediaType Type { get; init; }
	public required Size Dimensions { get; init; }

	public static MediaType GetMediaType(ReadOnlySpan<char> extension)
	{
		extension = extension.TrimStart('.');
		return extension switch
		{
			"gif" or "avi" or "mp4" or "mkv" => MediaType.Video,
			"jpg" or "jpeg" or "png" or "webp" => MediaType.Image,
			_ => MediaType.Unknown,
		};
	}
}