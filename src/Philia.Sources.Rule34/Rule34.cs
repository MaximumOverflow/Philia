using System.Text.Json.Serialization;
using System.Drawing;
using System.Text;

namespace Philia.Sources.Rule34;

public sealed class Rule34(HttpClient client) : Source(client), ISearchPosts
{
	public override string Name => "Rule34";
	
	public async Task<Philia.Post[]> SearchPosts(uint page, uint limit, PostOrder order, IEnumerable<string> include, IEnumerable<string> exclude)
	{
		var postOrder = order switch
		{
			PostOrder.Default => "",
			PostOrder.Newest => "+sort:id:desc",
			PostOrder.Oldest => "+sort:id:asc",
			PostOrder.MostLiked => "+sort:score:desc",
			PostOrder.LeastLiked => "+sort:score:asc",
			_ => throw new ArgumentOutOfRangeException(nameof(order), order, null),
		};

		var tags = new StringBuilder(postOrder);
		foreach (var tag in include.Distinct()) tags.Append($"+{tag}");
		foreach (var tag in exclude.Distinct()) tags.Append($"+-{tag}");
		var searchUrl = $"https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&limit={limit}&pid={page}&tags={tags}";
		
		var results = await FetchJsonObject<Post[]>(searchUrl);
		var posts = new Philia.Post[results.Length];
		var source = GetType().FullName ?? GetType().Name;

		var media = new List<Media>();
		
		for (var i = 0; i < results.Length; i++)
		{
			media.Clear();
			var post = results[i];

			if (post.PreviewUrl is not null)
			{
				media.Add(new Media
				{
					Url = post.PreviewUrl,
					Original = false,
					Type = Media.GetMediaType(Path.GetExtension(post.PreviewUrl.AsSpan())),
					Dimensions = default,
				});
			}
			
			if (post.SampleUrl is not null)
			{
				media.Add(new Media
				{
					Url = post.SampleUrl,
					Original = false,
					Type = Media.GetMediaType(Path.GetExtension(post.SampleUrl.AsSpan())),
					Dimensions = new Size(post.SampleWidth, post.SampleHeight),
				});
			}
			
			if (post.FileUrl is not null)
			{
				media.Add(new Media
				{
					Url = post.FileUrl,
					Original = true,
					Type = Media.GetMediaType(Path.GetExtension(post.FileUrl.AsSpan())),
					Dimensions = new Size(post.Width, post.Height),
				});
			}
			
			posts[i] = new Philia.Post
			{
				Source = source,
				Hash = post.Hash,
				Id = post.Id,
				Media = media.ToArray(),
				Rating = post.Rating switch
				{
					"safe" => Rating.Safe,
					"questionable" => Rating.Questionable,
					"explicit" => Rating.Explicit,
					_ => Rating.Unknown,
				},
				Score = post.Score,
				Tags = new TagCollection((post.Tags ?? string.Empty).Split(' ')),
			};
		}

		return posts;
	}
}

file record struct Post
{
	public ulong Id { get; init; }
	public string? Hash { get; init; }
	public string? Rating { get; init; }
	public string? Tags { get; init; }
	public long Score { get; init; }
	
	[JsonPropertyName("sample_url")] 
	public string? SampleUrl { get; init; }
	[JsonPropertyName("preview_url")] 
	public string? PreviewUrl { get; init; }
	[JsonPropertyName("file_url")] 
	public string? FileUrl { get; init; }
	
	public int Width { get; }
	public int Height { get; }
	[JsonPropertyName("sample_width")] 
	public int SampleWidth { get; }
	[JsonPropertyName("sample_height")] 
	public int SampleHeight { get; }
}