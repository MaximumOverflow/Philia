using System.Text.Json.Serialization;
using System.Collections.Frozen;
using System.Drawing;

namespace Philia.Sources;

public sealed class Danbooru(HttpClient client) : Booru(client, "https://danbooru.donmai.us")
{
	public override string Name => "Danbooru";
}

public sealed class Testbooru(HttpClient client) : Booru(client, "https://testbooru.donmai.us")
{
	public override string Name => "Testbooru";
}

public abstract partial class Booru : Source, ISearchPosts, IGetTags, IGetAllTags
{
	private readonly string _url;
	public bool DirectTagRetrieval => false;
	protected Booru(HttpClient client, string url) 
		: base(client) => _url = url;

	public async Task<Philia.Post[]> SearchPosts(uint page, uint limit, PostOrder order, IEnumerable<string> include, IEnumerable<string> exclude)
	{
		var postOrder = order switch
		{
			PostOrder.Default => "",
			PostOrder.Newest => "order:id_desc",
			PostOrder.Oldest => "order:id_asc",
			PostOrder.MostLiked => "order:score_desc",
			PostOrder.LeastLiked => "order:score_asc",
			_ => throw new ArgumentOutOfRangeException(nameof(order), order, null),
		};

		var includeTags = string.Join('+', include.Distinct());
		var excludeTags = string.Join("+-", exclude.Distinct());
		var searchUrl = $"{_url}/posts.json?limit={limit}&page={page}&tags={postOrder}";
		if (!string.IsNullOrWhiteSpace(includeTags)) searchUrl = $"{searchUrl}+{includeTags}";
		if (!string.IsNullOrWhiteSpace(excludeTags)) searchUrl = $"{searchUrl}+-{excludeTags}";

		var results = await FetchJsonObject<Post[]>(searchUrl);
		var posts = new Philia.Post[results.Length];
		
		var source = GetType().FullName ?? GetType().Name;

		for (var i = 0; i < results.Length; i++)
		{
			var post = results[i];
			
			var variants = post.Media?.Variants ?? Array.Empty<Variant>();
			var media = new Media[variants.Length];
			for (var j = 0; j < variants.Length; j++)
			{
				if(variants[j] is not { Url: {} url  } variant) continue;
				media[j] = new Media
				{
					Url = url,
					Original = variant.Type == "original",
					Type = Media.GetMediaType(variant.FileExt),
					Dimensions = new Size { Width = variant.Width ?? 0, Height = variant.Height ?? 0 },
				};
			}

			var tagCategories = new List<KeyValuePair<string, FrozenSet<string>>>();
			void ProcessTagCategory(string key, string? value)
			{
				if(string.IsNullOrWhiteSpace(value)) return;
				var tags = value.Split(' ').ToFrozenSet();
				tagCategories.Add(new KeyValuePair<string, FrozenSet<string>>(key, tags));
			}
			ProcessTagCategory("Meta", post.Meta);
			ProcessTagCategory("Artist", post.Artist);
			ProcessTagCategory("General", post.General);
			ProcessTagCategory("Character", post.Character);
			ProcessTagCategory("Copyright", post.Copyright);

			posts[i] = new Philia.Post
			{
				Source = source,
				Hash = post.Md5,
				Id = post.Id ?? 0,
				Media = media,
				Rating = post.Rating switch
				{
					"g" => Rating.General,
					"s" => Rating.Safe,
					"q" => Rating.Questionable,
					"e" => Rating.Explicit,
					_ => Rating.Unknown,
				},
				Score = post.Score ?? 0,
				Tags = new TagCollection(tagCategories.ToFrozenDictionary()),
			};
		}

		return posts;
	}

	public async Task<Philia.Tag[]> GetTags(uint page, uint limit, TagOrder order)
	{
		var tagOrder = order switch
		{
			TagOrder.Date => "id",
			TagOrder.Name => "name",
			TagOrder.Count => "count",
			_ => throw new ArgumentOutOfRangeException(nameof(order), order, null),
		};
		var searchUrl = $"{_url}/posts.json?limit={limit}&page={page}&search[order]={tagOrder}";
		
		var results = await FetchJsonObject<Tag[]>(searchUrl);
		var tags = new Philia.Tag[results.Length];
		for (var i = 0; i < results.Length; i++)
		{
			var tag = results[i];
			tags[i] = new Philia.Tag
			{
				Count = tag.Count ?? 0,
				Id = tag.Id ?? 0,
				Name = tag.Name,
			};
		}

		return tags;
	}
	
	public async Task<IList<Philia.Tag>> GetAllTags(TagOrder order, Action<IReadOnlyList<Philia.Tag>>? callback, CancellationToken cancellationToken)
	{
		var tags = new List<Philia.Tag>();
		for (uint i = 1; i <= 1000U; ++i)
		{
			tags.AddRange(await GetTags(i, int.MaxValue, order));
			callback?.Invoke(tags);
			if (cancellationToken.IsCancellationRequested)
				return tags;
		}
		return tags;
	}
}

file record struct Post
{
	public ulong? Id { get; init; }
	public long? Score { get; init; }
	public string? Rating { get; init; }
	public string? Tags { get; init; }
	public string? Md5 { get; init; }

	[JsonPropertyName("file_url")] 
	public string? ResourceFileUrl { get; init; }

	[JsonPropertyName("preview_file_url")] 
	public string? PreviewFileUrl { get; init; }

	[JsonPropertyName("image_width")] 
	public uint? ImageWidth { get; init; }

	[JsonPropertyName("image_height")] 
	public uint? ImageHeight { get; init; }

	[JsonPropertyName("media_asset")] 
	public MediaAsset? Media { get; init; }

	[JsonPropertyName("tag_string_meta")] 
	public string? Meta { get; init; }

	[JsonPropertyName("tag_string_artist")]
	public string? Artist { get; init; }

	[JsonPropertyName("tag_string_general")]
	public string? General { get; init; }

	[JsonPropertyName("tag_string_character")]
	public string? Character { get; init; }

	[JsonPropertyName("tag_string_copyright")]
	public string? Copyright { get; init; }
}

file record struct MediaAsset
{
	public Variant[]? Variants { get; init; }
}

file record struct Variant
{
	public string? Type { get; init; }
	public int? Width { get; init; }
	public int? Height { get; init; }
	public string? Url { get; init; }
	[JsonPropertyName("file_ext")]
	public string? FileExt { get; init; }
}

file record struct Tag
{
	[JsonPropertyName("post_count")] 
	public uint? Count { get; init; }
	public ulong? Id { get; init; }
	public string Name { get; init; }
}
