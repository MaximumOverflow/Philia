using System.Diagnostics.CodeAnalysis;
using System.Runtime.Serialization;
using System.Reflection;
using System.Text.Json;

namespace Philia;

public abstract class Source
{
	public static readonly JsonSerializerOptions JsonSerializerOptions = new()
	{
		AllowTrailingCommas = true,
		PropertyNameCaseInsensitive = true,
	};
	
	private readonly HttpClient _client;
	public abstract string Name { get; }
	public FeatureFlags FeatureFlags { get; }

	[SuppressMessage("ReSharper", "SuspiciousTypeConversion.Global")]
	protected Source(HttpClient client)
	{
		_client = client;
		FeatureFlags = FeatureFlags.None;
		if (this is IGetTags) FeatureFlags |= FeatureFlags.GetTags;
		if (this is IGetAllTags) FeatureFlags |= FeatureFlags.GetAllTags;
		if (this is ISearchTags) FeatureFlags |= FeatureFlags.SearchTags;
		if (this is ISearchPosts) FeatureFlags |= FeatureFlags.SearchPosts;
	}
	
	public bool HasFeature(FeatureFlags feature) => (FeatureFlags & feature) == feature;
	
	public async ValueTask<T> FetchJsonObject<T>(string url, JsonSerializerOptions? jsonSerializerOptions = null)
	{
		jsonSerializerOptions ??= JsonSerializerOptions;
		var json = await _client.GetStreamAsync(url);
		var obj = await JsonSerializer.DeserializeAsync<T>(json, jsonSerializerOptions);
		return obj ?? throw new SerializationException();
	}
	
	public static IReadOnlyList<Source> GetSources(Assembly assembly)
	{
		var sources = new List<Source>();
		foreach (var type in assembly.GetTypes())
		{
			if (type is not { IsAbstract: false, IsInterface: false }) 
				continue;

			if (!type.IsAssignableTo(typeof(Source))) 
				continue;
			
			try
			{
				if (Activator.CreateInstance(type) is Source instance)
					sources.Add(instance);
			}
			catch (Exception) { /*ignored*/ }
		}
		return sources;
	}
}

[Flags]
public enum FeatureFlags
{
	None = 0,
	GetTags = 1,
	GetAllTags = 2,
	SearchTags = 4,
	SearchPosts = 8,
}

public enum PostOrder
{
	Default,
	Newest,
	Oldest,
	MostLiked,
	LeastLiked,
}

public enum TagOrder
{
	Date,
	Name,
	Count,
}

public interface ISearchPosts
{
	public Task<Post[]> SearchPosts(uint page, uint limit, PostOrder order, IEnumerable<string> include, IEnumerable<string> exclude);
}

public interface ISearchTags
{
	public Task<Tag[]> SearchTags(uint page, uint limit, TagOrder order, string query);
}

public interface IGetTags
{
	public Task<Tag[]> GetTags(uint page, uint limit, TagOrder order);
}

public interface IGetAllTags
{
	public Task<IList<Tag>> GetAllTags(TagOrder order, Action<IReadOnlyList<Tag>>? callback, CancellationToken cancellationToken);
	
	public Task<IList<Tag>> GetAllTags(TagOrder order) => GetAllTags(order, null, CancellationToken.None);
	public Task<IList<Tag>> GetAllTags(TagOrder order, CancellationToken cancellationToken) => GetAllTags(order, null, cancellationToken);
	public Task<IList<Tag>> GetAllTags(TagOrder order, Action<IReadOnlyList<Tag>> callback) => GetAllTags(order, callback, CancellationToken.None);
}

public interface IEmbeddedTags
{
	public Tag[] EmbeddedTags { get; }
}
