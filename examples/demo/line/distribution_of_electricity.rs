use echarts::{
    component::{axis, title, toolbox, tooltip, visual_map},
    element::{
        AxisLabel, AxisPointer, AxisPointerType, AxisType, ItemStyle, MarkArea, MarkAreaData,
        TooltipTrigger,
    },
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            title::Title::new()
                .text("Distribution of Electricity")
                .subtext("Fake Data"),
        )
        .tooltip(
            tooltip::Tooltip::new()
                .trigger(TooltipTrigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
        )
        .toolbox(
            toolbox::Toolbox::new().show(true).feature(
                toolbox::Feature::new().save_as_image(toolbox::SaveAsImage::new().show(true)),
            ),
        )
        .x_axis(
            axis::Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec![
                    "00:00", "01:15", "02:30", "03:45", "05:00", "06:15", "07:30", "08:45",
                    "10:00", "11:15", "12:30", "13:45", "15:00", "16:15", "17:30", "18:45",
                    "20:00", "21:15", "22:30", "23:45",
                ]),
        )
        .y_axis(
            axis::Axis::new()
                .type_(AxisType::Value)
                .axis_label(AxisLabel::new().formatter("{value} W"))
                .axis_pointer(AxisPointer::new().snap(true)),
        )
        .visual_map(
            visual_map::VisualMap::new()
                .show(false)
                .dimension(0)
                .pieces(vec![
                    visual_map::Piece::new().lte(6).color("green"),
                    visual_map::Piece::new().gt(6).lte(8).color("red"),
                    visual_map::Piece::new().gt(8).lte(14).color("green"),
                    visual_map::Piece::new().gt(14).lte(17).color("red"),
                    visual_map::Piece::new().gt(17).color("green"),
                ]),
        )
        .series(Series::Line(
            line::Line::new()
                .name("Electricity")
                .smooth(0.5)
                .data(vec![
                    300, 280, 250, 260, 270, 300, 550, 500, 400, 390, 380, 390, 400, 500, 600, 750,
                    800, 700, 600, 400,
                ])
                .mark_area(
                    MarkArea::new()
                        .item_style(ItemStyle::new().color("rgba(255, 173, 177, 0.4)"))
                        .data(vec![
                            (
                                MarkAreaData::new().name("Morning Peak").x_axis("07:30"),
                                MarkAreaData::new().x_axis("10:00"),
                            ),
                            (
                                MarkAreaData::new().name("Evening Peak").x_axis("17:30"),
                                MarkAreaData::new().x_axis("21:15"),
                            ),
                        ]),
                ),
        ))
}
