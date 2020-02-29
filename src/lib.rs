extern crate plotly;
extern crate nanoid;

use plotly::Plot;
use nanoid::nanoid;
use std::fs;

/// Outputs Inline Plotly Plot for Jupyter
pub fn show_plot(plot: Plot) {
    
    let plotly_file = "temp_plot.html";
    plot.to_html(plotly_file);
    let plotly_contents = fs::read_to_string(plotly_file).unwrap();
    fs::remove_file(plotly_file);

    let start_bytes = plotly_contents
        .find("<div id='plotly-html-element' class='plotly-graph-div'")
        .unwrap_or(0);
    
    let end_bytes = plotly_contents
        .find("\n</div>\n</body>\n</html>")
        .unwrap_or(plotly_contents.len());
    
    println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT",
    format!("<div>{}</div>",
        &plotly_contents[start_bytes..end_bytes]
        .replace("plotly-html-element", Box::leak(nanoid!().into_boxed_str()))
        .replace("var layout = {", "var layout = {'annotationdefaults': {'arrowcolor': '#2a3f5f', 'arrowhead': 0, 'arrowwidth': 1},
               'colorscale': {'diverging': [[0, '#8e0152'], [0.1, '#c51b7d'],
                                            [0.2, '#de77ae'], [0.3, '#f1b6da'],
                                            [0.4, '#fde0ef'], [0.5, '#f7f7f7'],
                                            [0.6, '#e6f5d0'], [0.7, '#b8e186'],
                                            [0.8, '#7fbc41'], [0.9, '#4d9221'], [1,
                                            '#276419']],
                              'sequential': [[0.0, '#0d0887'],
                                             [0.1111111111111111, '#46039f'],
                                             [0.2222222222222222, '#7201a8'],
                                             [0.3333333333333333, '#9c179e'],
                                             [0.4444444444444444, '#bd3786'],
                                             [0.5555555555555556, '#d8576b'],
                                             [0.6666666666666666, '#ed7953'],
                                             [0.7777777777777778, '#fb9f3a'],
                                             [0.8888888888888888, '#fdca26'], [1.0,
                                             '#f0f921']],
                              'sequentialminus': [[0.0, '#0d0887'],
                                                  [0.1111111111111111, '#46039f'],
                                                  [0.2222222222222222, '#7201a8'],
                                                  [0.3333333333333333, '#9c179e'],
                                                  [0.4444444444444444, '#bd3786'],
                                                  [0.5555555555555556, '#d8576b'],
                                                  [0.6666666666666666, '#ed7953'],
                                                  [0.7777777777777778, '#fb9f3a'],
                                                  [0.8888888888888888, '#fdca26'],
                                                  [1.0, '#f0f921']]},
               'colorway': ['#636efa', '#EF553B', '#00cc96', '#ab63fa', '#FFA15A', '#19d3f3',
                            '#FF6692', '#B6E880', '#FF97FF', '#FECB52'],
               'font': {'color': '#2a3f5f'},
               'geo': {'bgcolor': 'white',
                       'lakecolor': 'white',
                       'landcolor': '#E5ECF6',
                       'showlakes': true,
                       'showland': true,
                       'subunitcolor': 'white'},
               'hoverlabel': {'align': 'left'},
               'hovermode': 'closest',
               'legend': {'orientation': 'h', 'x': 0.5, 'xanchor': 'center', 'y': 1.1},
               'mapbox': {'style': 'light'},
               'margin': {'b': 40, 'l': 40, 'r': 10, 't': 0},
               'paper_bgcolor': 'white',
               'plot_bgcolor': '#E5ECF6',
               'polar': {'angularaxis': {'gridcolor': 'white', 'linecolor': 'white', 'ticks': ''},
                         'bgcolor': '#E5ECF6',
                         'radialaxis': {'gridcolor': 'white', 'linecolor': 'white', 'ticks': ''}},
               'scene': {'xaxis': {'backgroundcolor': '#E5ECF6',
                                   'gridcolor': 'white',
                                   'gridwidth': 2,
                                   'linecolor': 'white',
                                   'showbackground': true,
                                   'ticks': '',
                                   'zerolinecolor': 'white'},
                         'yaxis': {'backgroundcolor': '#E5ECF6',
                                   'gridcolor': 'white',
                                   'gridwidth': 2,
                                   'linecolor': 'white',
                                   'showbackground': true,
                                   'ticks': '',
                                   'zerolinecolor': 'white'},
                         'zaxis': {'backgroundcolor': '#E5ECF6',
                                   'gridcolor': 'white',
                                   'gridwidth': 2,
                                   'linecolor': 'white',
                                   'showbackground': true,
                                   'ticks': '',
                                   'zerolinecolor': 'white'}},
               'shapedefaults': {'line': {'color': '#2a3f5f'}},
               'ternary': {'aaxis': {'gridcolor': 'white', 'linecolor': 'white', 'ticks': ''},
                           'baxis': {'gridcolor': 'white', 'linecolor': 'white', 'ticks': ''},
                           'bgcolor': '#E5ECF6',
                           'caxis': {'gridcolor': 'white', 'linecolor': 'white', 'ticks': ''}},
               'title': {'x': 0.05},")
    ));
}