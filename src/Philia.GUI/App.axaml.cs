using System.Net.Http;

namespace Philia.GUI;

public partial class App : Application
{
	public static readonly HttpClient HttpClient;
	public const string DownloadDir = "Downloads";

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
				desktop.Exit += (_, _) => viewModel.Dispose();
				break;
			
			case ISingleViewApplicationLifetime singleViewPlatform:
				singleViewPlatform.MainView = new MainView { DataContext = viewModel };
				singleViewPlatform.MainView.Unloaded += (_, _) => viewModel.Dispose();
				break;
		}
		
		base.OnFrameworkInitializationCompleted();
	}
}