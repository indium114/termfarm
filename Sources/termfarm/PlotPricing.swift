import Foundation

let basePlotPrice   = 100
let plotPriceGrowth = 1.5

func nextPlotPrice(plotCount: Int) -> Int {
    Int(Double(basePlotPrice) * pow(plotPriceGrowth, Double(plotCount - 3)))
}
