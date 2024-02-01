using System.Text.Json.Serialization;
using System.Collections.Frozen;
using System.Collections;
using System.Text.Json;

namespace Philia;

public readonly record struct Tag(ulong Id, uint Count, string Name);

[JsonConverter(typeof (TagCollectionJsonConverter))]
public sealed class TagCollection : IEnumerable<string>, IEnumerable<KeyValuePair<string, FrozenSet<string>>>
{
	private FrozenSet<string>? _tags;
	public FrozenDictionary<string, FrozenSet<string>> TagCategories { get; }

	[JsonIgnore]
	public FrozenSet<string> Tags
	{
		get
		{
			if (_tags is not null)	
				return _tags;
			
			if (!Uncategorized)
			  return _tags = TagCategories.Values.SelectMany( s => s).ToFrozenSet();

			if (!TagCategories.TryGetValue("null", out _tags))
				_tags = FrozenSet<string>.Empty;

			return _tags;
		}
	}

	public TagCollection()
	{
		TagCategories = FrozenDictionary<string, FrozenSet<string>>.Empty;
	}

	public TagCollection(IEnumerable<string> tags)
	{
		_tags = tags.ToFrozenSet();
		TagCategories = FrozenDictionary<string, FrozenSet<string>>.Empty;
	}

	public TagCollection(FrozenDictionary<string, FrozenSet<string>> tagCategories)
	{	
		TagCategories = tagCategories;
	}

	[JsonIgnore]
	public int Count => Tags.Count;

	[JsonIgnore]
	public bool Uncategorized
	{
		get
		{
			if (TagCategories.Count == 0) return true;
			return TagCategories.Count == 1 && TagCategories.ContainsKey("null");
		}
	}
	
	public FrozenSet<string>.Enumerator GetEnumerator()
		=> Tags.GetEnumerator();

	IEnumerator<string> IEnumerable<string>.GetEnumerator()
		=> Tags.GetEnumerator();
	
	IEnumerator IEnumerable.GetEnumerator()
		=> Tags.GetEnumerator();
	
	IEnumerator<KeyValuePair<string, FrozenSet<string>>> IEnumerable<KeyValuePair<string, FrozenSet<string>>>.GetEnumerator()
		=> TagCategories.GetEnumerator();
}

internal sealed class TagCollectionJsonConverter : JsonConverter<TagCollection>
{
	private const StringSplitOptions SplitOptions =
		StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;
	
	public override TagCollection Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
	{
		if (reader.TokenType == JsonTokenType.String)
		{
			var str = reader.GetString();
			return str != null 
				? new TagCollection(str.Split(' ', SplitOptions)) 
				: new TagCollection();
		}
		
		if (reader.TokenType != JsonTokenType.StartObject)
			throw new JsonException();

		var tagCategories = new List<KeyValuePair<string, FrozenSet<string>>>();
		
		while (reader.Read())
		{
			switch (reader.TokenType)
			{
				case JsonTokenType.PropertyName:
				{
					var key = reader.GetString() ?? throw new JsonException();
					reader.Read();
					var value = reader.GetString() ?? string.Empty;
					var tags = value.Split(' ', SplitOptions).ToFrozenSet();
					tagCategories.Add(new KeyValuePair<string, FrozenSet<string>>(key, tags));
					break;
				}
					
				case JsonTokenType.EndObject:
					return new TagCollection(tagCategories.ToFrozenDictionary());
				
				default: 
					throw new JsonException();
			}
		}
		
		throw new JsonException();
	}

	public override void Write(Utf8JsonWriter writer, TagCollection value, JsonSerializerOptions options)
	{
		if (value.Uncategorized)
		{
			var str = string.Join(' ', value.Tags);
			writer.WriteStringValue(str);
			return;
		}
		
		writer.WriteStartObject();
		foreach (var (key, tags) in value.TagCategories)
			writer.WriteString(key, string.Join(' ', tags));
		writer.WriteEndObject();
	}
}
