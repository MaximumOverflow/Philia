using System.Text.Json.Serialization;
using System.Drawing;

namespace Philia.Sources;

public sealed class Safebooru : Source, ISearchPosts
{
	public override string Name => "Safebooru";

	public Safebooru(HttpClient client) 
		: base(client) {}

	public async Task<Philia.Post[]> SearchPosts(uint page, uint limit, PostOrder order, IEnumerable<string> include, IEnumerable<string> exclude)
	{
		var postOrder = order switch
		{
			PostOrder.Default => "",
			PostOrder.Newest => "order:id:desc",
			PostOrder.Oldest => "order:id:asc",
			PostOrder.MostLiked => "order:score:desc",
			PostOrder.LeastLiked => "order:score:asc",
			_ => throw new ArgumentOutOfRangeException(nameof(order), order, null),
		};
		
		var includeTags = string.Join('+', include.Distinct());
		var excludeTags = string.Join("+-", exclude.Distinct());
		var searchUrl = $"https://safebooru.org/index.php?page=dapi&s=post&q=index&json=1&limit={limit}&pid={page}&tags={postOrder}";
		if (!string.IsNullOrWhiteSpace(includeTags)) searchUrl = $"{searchUrl}+{includeTags}";
		if (!string.IsNullOrWhiteSpace(excludeTags)) searchUrl = $"{searchUrl}+-{excludeTags}";

		var results = await FetchJsonObject<Post[]>(searchUrl);
		var posts = new Philia.Post[results.Length];

		var source = typeof(Safebooru).FullName ?? nameof(Safebooru);
		
		for (var i = 0; i < results.Length; i++)
		{
			var post = results[i];
			var media = Array.Empty<Media>();
			
			if (post.Image is {} imageUrl)
			{
				media = [
					new Media
					{
						Url = $"https://safebooru.org/thumbnails/{post.Directory}/thumbnail_{Path.ChangeExtension(post.Image, ".jpg")}",
						Original = false,
						Type = Media.GetMediaType(Path.GetExtension(imageUrl.AsSpan())),
						Dimensions = default,
					},
					new Media
					{
						Url = $"https://safebooru.org/images/{post.Directory}/{post.Image}",
						Original = true,
						Type = Media.GetMediaType(Path.GetExtension(imageUrl.AsSpan())),
						Dimensions = new Size((int)(post.Width ?? 0), (int)(post.Height ?? 0)),
					},
				];
			}
			
			posts[i] = new Philia.Post
			{
				Source = source,
				Hash = post.Hash,
				Id = post.Id ?? 0,
				Rating = post.Rating switch
				{
					"general" => Rating.General,
					"safe" => Rating.Safe,
					"questionable" => Rating.Questionable,
					"explicit" => Rating.Explicit,
					"sensitive" => Rating.Sensitive,
					_ => Rating.Unknown,
				},
				Score = post.Score ?? 0,
				Tags = new TagCollection((post.Tags ?? string.Empty).Split(' ')),
				Media = media,
			};
		}

		return posts;
	}
}

file record struct Post
{
	public ulong? Id { get; init; }
	public long? Score { get; init; }
	public string? Rating { get; init; }
	public string? Tags { get; init; }
	public string? Hash { get; init; }
	public string? Image { get; init; }
	public string? Directory { get; init; }
	public uint? Width { get; init; }
	public uint? Height { get; init; }

	[JsonPropertyName("sample_width")] 
	public uint? SampleWidth { get; init; }

	[JsonPropertyName("sample_height")] 
	public uint? SampleHeight { get; init; }
}