use echarts::{
    component::axis,
    element::AxisType,
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(AxisType::Value))
        .series(Series::Line(
            line::Line::new()
                .smooth(0.5)
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        ))
}
