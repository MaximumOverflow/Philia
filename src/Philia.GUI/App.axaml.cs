using System.Net.Http;

namespace Philia.GUI;

public partial class App : Application
{
	public static readonly HttpClient HttpClient;

	static App()
	{
		var assemblyVer = typeof(App).Assembly.GetName().Version?.ToString() ?? "unknown";
		HttpClient = new HttpClient();
		HttpClient.DefaultRequestHeaders.UserAgent.ParseAdd($"PhiliaGUI/{assemblyVer}");
	}
	
	public override void Initialize()
	{
		AvaloniaXamlLoader.Load(this);
	}

	public override void OnFrameworkInitializationCompleted()
	{
		var plugins = Plugin.LoadPlugins("Plugins");
		var viewModel = new MainViewModel(plugins);
		
		switch (ApplicationLifetime)
		{
			case IClassicDesktopStyleApplicationLifetime desktop:
				desktop.MainWindow = new MainWindow { DataContext = viewModel };
				break;
			
			case ISingleViewApplicationLifetime singleViewPlatform:
				singleViewPlatform.MainView = new MainView { DataContext = viewModel };
				break;
		}

		base.OnFrameworkInitializationCompleted();
	}
}