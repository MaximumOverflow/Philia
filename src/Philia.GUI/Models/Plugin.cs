using System.Collections.Generic;
using System.Reflection;
using System.IO;
using System.Net.Http;

namespace Philia.GUI.ViewModels;

public sealed class Plugin
{
	public string Name { get; }
	public Assembly Assembly { get; }
	public IReadOnlyList<Source> Sources { get; }
	public IReadOnlyList<Type> FailedToLoad { get; }
	
	private Plugin(Assembly assembly, IReadOnlyList<Source> sources, IReadOnlyList<Type> failedToLoad)
	{
		Sources = sources;
		Assembly = assembly;
		FailedToLoad = failedToLoad;
		Name = assembly.GetName().Name ?? assembly.GetName().FullName;
	}

	public static Plugin? TryAsPlugin(Assembly assembly)
	{
		var failed = new List<Type>();
		var sources = new List<Source>();
		foreach (var type in assembly.DefinedTypes)
		{
			if(type.IsAbstract || !type.IsAssignableTo(typeof(Source)))
				continue;

			try
			{
				if (Activator.CreateInstance(type, App.HttpClient) is Source source)
					sources.Add(source);
				else
					failed.Add(type);
			}
			catch (Exception e)
			{
				Console.Error.WriteLine(e);
				failed.Add(type);
			}
		}

		if (sources.Count == 0 && failed.Count == 0)
			return null;

		return new Plugin(assembly, sources, failed);
	}

	public static IEnumerable<Plugin> LoadPlugins(string folder)
	{
		if (!Directory.Exists(folder))
			return Array.Empty<Plugin>();
		
		var plugins = new List<Plugin>();
		var pluginDirectory = new DirectoryInfo(folder);
		foreach (var assemblyPath in pluginDirectory.EnumerateFiles("*.dll", SearchOption.AllDirectories))
		{
			var assembly = Assembly.LoadFile(assemblyPath.FullName);
			if(TryAsPlugin(assembly) is {} plugin) plugins.Add(plugin);
		}

		return plugins;
	}
}