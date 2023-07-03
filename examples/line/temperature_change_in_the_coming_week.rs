use echarts::{
    component::{axis, legend, title, toolbox, tooltip},
    element::{label, symbol::Symbol},
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .title(title::Title::new().text("Temperature change in the coming week"))
        .tooltip(tooltip::Tooltip::new().trigger(tooltip::Trigger::Axis))
        .legend(legend::Legend::new())
        .toolbox(
            toolbox::Toolbox::new().show(true).feature(
                toolbox::Feature::new()
                    .save_as_image(toolbox::SaveAsImage::new())
                    .restore(toolbox::Restore::new())
                    .magic_type(toolbox::MagicType::new().type_(vec![
                        toolbox::MagicTypeType::Line,
                        toolbox::MagicTypeType::Bar,
                    ]))
                    .data_zoom(toolbox::DataZoom::new().y_axis_index("none"))
                    .data_view(toolbox::DataView::new().read_only(false)),
            ),
        )
        .x_axis(
            axis::Axis::new()
                .type_(axis::Type::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(
            axis::Axis::new()
                .type_(axis::Type::Value)
                .axis_label(axis::AxisLabel::new().formatter("{value} °C")),
        )
        .series(Series::Line(
            line::Line::new()
                .name("Highest")
                .data(vec![10, 11, 13, 11, 12, 12, 9])
                .mark_point(line::MarkPoint::new().data(vec![("max", "Max"), ("min", "Min")]))
                .mark_line(
                    line::MarkLine::new().data(vec![line::MarkLineVariant::Simple(
                        line::MarkLineData::new()
                            .type_(line::MarkLineDataType::Average)
                            .name("Avg"),
                    )]),
                ),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Lowest")
                .data(vec![1, -2, 2, 5, 3, 2, 0])
                .mark_point(line::MarkPoint::new().data(vec![line::MarkPointData::new()
                        .name("周最低")
                        .value(-2)
                        .x_axis(1)
                        .y_axis(-1.5)]))
                .mark_line(line::MarkLine::new().data(vec![
                        line::MarkLineVariant::Simple(
                            line::MarkLineData::new()
                                .type_(line::MarkLineDataType::Average)
                                .name("Avg"),
                        ),
                        line::MarkLineVariant::StartToEnd(
                            line::MarkLineData::new()
                                .symbol(Symbol::None)
                                .x("90%")
                                .y_axis("max"),
                            line::MarkLineData::new()
                                .type_(line::MarkLineDataType::Max)
                                .name("最高点")
                                .symbol(Symbol::Circle)
                                .label(
                                    label::Label::new()
                                        .position(label::Position::Start)
                                        .formatter("Max"),
                                )
                        ),
                    ])),
        ));

    println!("{}", chart.to_string());
}
