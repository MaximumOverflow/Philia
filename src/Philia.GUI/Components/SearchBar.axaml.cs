using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Globalization;
using System.Threading.Tasks;
using Avalonia.Data.Converters;
using Avalonia.Input;
using Avalonia.Interactivity;
using Avalonia.Media;

namespace Philia.GUI.Views;

public partial class SearchBar : UserControl
{
	public static readonly StyledProperty<ISearchBarBehaviour?> BehaviourProperty =
		AvaloniaProperty.Register<SearchBar, ISearchBarBehaviour?>(nameof(Behaviour));
	
	public ISearchBarBehaviour? Behaviour
	{
		get => GetValue(BehaviourProperty);
		set => SetValue(BehaviourProperty, value);
	}
	
	public SearchBar()
	{
		InitializeComponent();
		Input.AddHandler(KeyDownEvent, OnKeyDown, RoutingStrategies.Tunnel);
	}

	protected void OnKeyDown(object? sender, KeyEventArgs e)
	{
		if(DataContext is not ISearchBarContext {Query: var tags})
			return;
		
		switch (e)
		{
			case { Key: Key.Enter, KeyModifiers: KeyModifiers.Control }:
			case { Key: Key.Enter, KeyModifiers: KeyModifiers.None } when string.IsNullOrWhiteSpace(Input.Text):
			{
				Behaviour?.Search(DataContext, tags).ConfigureAwait(false);
				e.Handled = true;
				break;
			}

			case { Key: Key.Space or Key.Enter, KeyModifiers: KeyModifiers.None }:
			{
				e.Handled = true;
				var tag = (Input.Text ?? string.Empty).Trim();
				tags.Add(tag);
				Input.Clear();
				break;
			}

			case { Key: Key.Back, KeyModifiers: KeyModifiers.None }
				when string.IsNullOrWhiteSpace(Input.Text) && tags.Count != 0:
			{
				e.Handled = true;
				var tag = tags[^1];
				tags.RemoveAt(tags.Count - 1);
				Input.Text = tag;
				Input.CaretIndex = int.MaxValue;
				break;
			}

			case { Key: Key.Back, KeyModifiers: KeyModifiers.Control } 
				when string.IsNullOrWhiteSpace(Input.Text) && tags.Count != 0:
			{
				e.Handled = true;
				tags.RemoveAt(tags.Count - 1);
				break;
			}

			default:
			{
				base.OnKeyDown(e);
				break;
			}
		}
	}
}

public class TagColorConverter : IValueConverter
{
	public static readonly TagColorConverter Instance = new();
	private static readonly SolidColorBrush Red = new(0xffd43d3d);
	private static readonly SolidColorBrush Green = new(0xff4eba3d);

	public object? Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
	{
		if (value is not string tag || targetType != typeof(IBrush))
			return null;

		return tag.StartsWith('-') ? Red : Green;
	}

	public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
	{
		throw new NotSupportedException();
	}
}

public interface ISearchBarContext
{
	public ObservableCollection<string> Query { get; }
}

public interface ISearchBarBehaviour
{
	public Task Search(object? context, IReadOnlyList<string> query);
}


