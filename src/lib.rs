extern crate plotly;
extern crate nanoid;
extern crate ndarray;

use plotly::Plot;
use nanoid::nanoid;
use std::fs;
use std::fmt::Debug;
use ndarray::prelude::*;


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
        .replace("height:100%; width:100%;", "min-height:450px; width:100%;")
        .replace("var layout = {", "var layout = {
            'annotationdefaults': {'arrowcolor': '#2a3f5f', 'arrowhead': 0, 'arrowwidth': 1},
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

pub fn show_array<T: Debug>(values: &Array2<T>) {
    let mut html = String::new();
    html.push_str("<table>");
    for r in 0..(values.shape()[0]) {
        html.push_str("<tr>");
        for c in 0..values.shape()[1] {
            html.push_str("<td>");
            html.push_str(&format!("{:?}", values[[r, c]]));
            html.push_str("</td>");
        }
        html.push_str("</tr>");            
    }
    html.push_str("</table>");
    println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", html);
}

pub fn show_frame<T: Debug>(values: &Array2<T>, headers: Option<&Vec<String>>) {
    let mut html = String::new();
    html.push_str("<table>");
    
    if let Some(x) = headers {
        html.push_str("<thead><tr style=\"text-align: right;\">"); 
        for c in 0..x.len() {
            html.push_str("<th>");
            html.push_str(&x[c]);
            html.push_str("</th>");
        }
        html.push_str("</tr></thead>");
    }
    
    html.push_str("<tbody>"); 
    if values.shape()[0] <= 11 {         
        for r in 0..(values.shape()[0]) {
            html.push_str("<tr>");
            for c in 0..values.shape()[1] {
                html.push_str("<td>");
                html.push_str(&format!("{:?}", values[[r, c]]));
                html.push_str("</td>");
            }
            html.push_str("</tr>");            
        }
    } else {          
        for r in 0..5 {
            html.push_str("<tr>");
            for c in 0..values.shape()[1] {
                html.push_str("<td>");
                html.push_str(&format!("{:?}", values.slice(s![0..5, ..])[[r, c]]));
                html.push_str("</td>");
            }
            html.push_str("</tr>");            
        }

        html.push_str("<tr>");
        for c in 0..values.shape()[1] {
            html.push_str("<td>");
            html.push_str(&format!("..."));
            html.push_str("</td>");
        }
        html.push_str("</tr>");   

        for r in 0..5 {
            html.push_str("<tr>");
            for c in 0..values.shape()[1] {
                html.push_str("<td>");
                html.push_str(&format!("{:?}", values.slice(s![-5.., ..])[[r, c]]));
                html.push_str("</td>");
            }
            html.push_str("</tr>");            
        }       
    }
    html.push_str("</tbody>");
    html.push_str("</table>");
    println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", html);
}

use std::io::prelude::*;
use ndarray_csv::Array2Reader;
use itertools::Itertools;
use std::str::FromStr;


pub fn read_csv(url : &str) -> (Array2<String>, Vec<String>) {
    let file_name = "temp.csv";

    let res = ureq::get(url).call().into_string().unwrap();

    let mut file = fs::File::create(file_name).unwrap();
    file.write_all(res.as_bytes());
    let mut rdr = csv::Reader::from_path(file_name).unwrap();
    fs::remove_file(file_name).unwrap();

    let data: Array2<String>= rdr.deserialize_array2_dynamic().unwrap();
    let mut headers : Vec<String> = Vec::new();
    for element in rdr.headers().unwrap().into_iter() {
        headers.push(String::from(element));
    };

    (data, headers)
}

pub fn iris_raw() -> (Array2<String>, Vec<String>) {
    let file_name = "Iris.csv";

    let res = ureq::get("https://shahinrostami.com/datasets/Iris.csv").call().into_string().unwrap();

    let mut file = fs::File::create(file_name).unwrap();
    file.write_all(res.as_bytes());
    let mut rdr = csv::Reader::from_path(file_name).unwrap();
    fs::remove_file(file_name).unwrap();

    let data: Array2<String>= rdr.deserialize_array2_dynamic().unwrap();
    let mut headers : Vec<String> = Vec::new();
    for element in rdr.headers().unwrap().into_iter() {
        headers.push(String::from(element));
    };

    (data, headers)
}

pub fn iris_typed() -> (Array2::<f32>, Vec<String>, Array1::<String>)
{
let raw_iris = iris_raw();
let data = raw_iris.0;
let headers = raw_iris.1;

let mut features: Array2::<f32> =  Array2::<f32>::zeros((data.shape()[0],0));

for &f in [1, 2, 3, 4].iter() {
    features = ndarray::stack![Axis(1), features,
        data.column(f as usize)
            .mapv(|elem| f32::from_str(&elem).unwrap())
            .insert_axis(Axis(1))];
};

let feature_headers = headers[1..5].to_vec();
let labels: Array1::<String> = data.column(5).to_owned();

(features, feature_headers, labels)
}

