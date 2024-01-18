// 1.提供 MarketUpdate, OrderGenerator, FillUpdater 这三个trait，用以处理
//
//
//



pub mod portfolio;


/// Update the [Portfolio] from an input [MarketEvent].
/// 根据传入的MarketEvent，来更新Portfolio.
pub trait MarketUpdate {
    /// If the [Portfolio] has an open [Position] relating to the input [MarketEvent],
    /// update it with the market data, and return [PositionUpdate] detailing the change.
    /// 如果Portfolio中开放了一个跟MarketEvent相关的Position，就更新该Portfolio，并以PositionUpdate结构返回详细的改动细节.
    fn update_from_market(
        &mut self,
        market: &MarketEvent,
    ) -> Result<Option<PositionUpdate>, PortfolioError>;
}

/// May generate an [OrderEvent] from an input [SignalEvent].
pub trait OrderGenerator {
    /// May genrate an [OrderEvent] after analysing the input [SignalEvent].
    fn generate_order(
        &mut self,
        signal: &SignalEvent,
    ) -> Result<Option<OrderEvent>, PortfolioError>;
}

/// Update the [Portfolio] from an input [FillEvent].
pub trait FillUpdater {
    /// Update the [Portfolio] with the input [FillEvent].
    /// The [FillEvent] triggers a [Position] entry or exit, and update key fields(such as current_cash and current_value) accordingly.
    /// 使用输入的FillEvent，来更新投资组合的内部状态.
    /// FillEvent会触发Position(持仓)的进入或退出，
}
