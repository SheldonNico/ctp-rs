#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// autobind.py generated.
pub trait Rust_CThostFtdcTraderSpi_Trait {
    fn on_front_connected(&mut self) {  }
    fn on_front_disconnected(&mut self, nReason: ::std::os::raw::c_int) {  }
    fn on_heart_beat_warning(&mut self, nTimeLapse: ::std::os::raw::c_int) {  }
    fn on_rsp_authenticate(&mut self, pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_user_login(&mut self, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_user_logout(&mut self, pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_user_password_update(&mut self, pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_trading_account_password_update(&mut self, pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_user_auth_method(&mut self, pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_gen_user_captcha(&mut self, pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_gen_user_text(&mut self, pRspGenUserText: *mut CThostFtdcRspGenUserTextField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_order_insert(&mut self, pInputOrder: *mut CThostFtdcInputOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_parked_order_insert(&mut self, pParkedOrder: *mut CThostFtdcParkedOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_parked_order_action(&mut self, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_order_action(&mut self, pInputOrderAction: *mut CThostFtdcInputOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_query_max_order_volume(&mut self, pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_settlement_info_confirm(&mut self, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_remove_parked_order(&mut self, pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_remove_parked_order_action(&mut self, pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_exec_order_insert(&mut self, pInputExecOrder: *mut CThostFtdcInputExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_exec_order_action(&mut self, pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_for_quote_insert(&mut self, pInputForQuote: *mut CThostFtdcInputForQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_quote_insert(&mut self, pInputQuote: *mut CThostFtdcInputQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_quote_action(&mut self, pInputQuoteAction: *mut CThostFtdcInputQuoteActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_batch_order_action(&mut self, pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_option_self_close_insert(&mut self, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_option_self_close_action(&mut self, pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_comb_action_insert(&mut self, pInputCombAction: *mut CThostFtdcInputCombActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_order(&mut self, pOrder: *mut CThostFtdcOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_trade(&mut self, pTrade: *mut CThostFtdcTradeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_investor_position(&mut self, pInvestorPosition: *mut CThostFtdcInvestorPositionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_trading_account(&mut self, pTradingAccount: *mut CThostFtdcTradingAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_investor(&mut self, pInvestor: *mut CThostFtdcInvestorField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_trading_code(&mut self, pTradingCode: *mut CThostFtdcTradingCodeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_instrument_margin_rate(&mut self, pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_instrument_commission_rate(&mut self, pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_exchange(&mut self, pExchange: *mut CThostFtdcExchangeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_product(&mut self, pProduct: *mut CThostFtdcProductField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_instrument(&mut self, pInstrument: *mut CThostFtdcInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_settlement_info(&mut self, pSettlementInfo: *mut CThostFtdcSettlementInfoField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_transfer_bank(&mut self, pTransferBank: *mut CThostFtdcTransferBankField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_investor_position_detail(&mut self, pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_notice(&mut self, pNotice: *mut CThostFtdcNoticeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_settlement_info_confirm(&mut self, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_investor_position_combine_detail(&mut self, pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_cfmmc_trading_account_key(&mut self, pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_e_warrant_offset(&mut self, pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_investor_product_group_margin(&mut self, pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_exchange_margin_rate(&mut self, pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_exchange_margin_rate_adjust(&mut self, pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_exchange_rate(&mut self, pExchangeRate: *mut CThostFtdcExchangeRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_sec_agent_acid_map(&mut self, pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_product_exch_rate(&mut self, pProductExchRate: *mut CThostFtdcProductExchRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_product_group(&mut self, pProductGroup: *mut CThostFtdcProductGroupField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_mm_instrument_commission_rate(&mut self, pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_mm_option_instr_comm_rate(&mut self, pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_instrument_order_comm_rate(&mut self, pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_sec_agent_trading_account(&mut self, pTradingAccount: *mut CThostFtdcTradingAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_sec_agent_check_mode(&mut self, pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_sec_agent_trade_info(&mut self, pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_option_instr_trade_cost(&mut self, pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_option_instr_comm_rate(&mut self, pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_exec_order(&mut self, pExecOrder: *mut CThostFtdcExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_for_quote(&mut self, pForQuote: *mut CThostFtdcForQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_quote(&mut self, pQuote: *mut CThostFtdcQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_option_self_close(&mut self, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_invest_unit(&mut self, pInvestUnit: *mut CThostFtdcInvestUnitField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_comb_instrument_guard(&mut self, pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_comb_action(&mut self, pCombAction: *mut CThostFtdcCombActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_transfer_serial(&mut self, pTransferSerial: *mut CThostFtdcTransferSerialField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_accountregister(&mut self, pAccountregister: *mut CThostFtdcAccountregisterField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_error(&mut self, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rtn_order(&mut self, pOrder: *mut CThostFtdcOrderField) {  }
    fn on_rtn_trade(&mut self, pTrade: *mut CThostFtdcTradeField) {  }
    fn on_err_rtn_order_insert(&mut self, pInputOrder: *mut CThostFtdcInputOrderField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_order_action(&mut self, pOrderAction: *mut CThostFtdcOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rtn_instrument_status(&mut self, pInstrumentStatus: *mut CThostFtdcInstrumentStatusField) {  }
    fn on_rtn_bulletin(&mut self, pBulletin: *mut CThostFtdcBulletinField) {  }
    fn on_rtn_trading_notice(&mut self, pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField) {  }
    fn on_rtn_error_conditional_order(&mut self, pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField) {  }
    fn on_rtn_exec_order(&mut self, pExecOrder: *mut CThostFtdcExecOrderField) {  }
    fn on_err_rtn_exec_order_insert(&mut self, pInputExecOrder: *mut CThostFtdcInputExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_exec_order_action(&mut self, pExecOrderAction: *mut CThostFtdcExecOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_for_quote_insert(&mut self, pInputForQuote: *mut CThostFtdcInputForQuoteField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rtn_quote(&mut self, pQuote: *mut CThostFtdcQuoteField) {  }
    fn on_err_rtn_quote_insert(&mut self, pInputQuote: *mut CThostFtdcInputQuoteField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_quote_action(&mut self, pQuoteAction: *mut CThostFtdcQuoteActionField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {  }
    fn on_rtn_cfmmc_trading_account_token(&mut self, pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField) {  }
    fn on_err_rtn_batch_order_action(&mut self, pBatchOrderAction: *mut CThostFtdcBatchOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rtn_option_self_close(&mut self, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField) {  }
    fn on_err_rtn_option_self_close_insert(&mut self, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_option_self_close_action(&mut self, pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rtn_comb_action(&mut self, pCombAction: *mut CThostFtdcCombActionField) {  }
    fn on_err_rtn_comb_action_insert(&mut self, pInputCombAction: *mut CThostFtdcInputCombActionField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rsp_qry_contract_bank(&mut self, pContractBank: *mut CThostFtdcContractBankField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_parked_order(&mut self, pParkedOrder: *mut CThostFtdcParkedOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_parked_order_action(&mut self, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_trading_notice(&mut self, pTradingNotice: *mut CThostFtdcTradingNoticeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_broker_trading_params(&mut self, pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_qry_broker_trading_algos(&mut self, pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_query_cfmmc_trading_account_token(&mut self, pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rtn_from_bank_to_future_by_bank(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField) {  }
    fn on_rtn_from_future_to_bank_by_bank(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField) {  }
    fn on_rtn_repeal_from_bank_to_future_by_bank(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField) {  }
    fn on_rtn_repeal_from_future_to_bank_by_bank(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField) {  }
    fn on_rtn_from_bank_to_future_by_future(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField) {  }
    fn on_rtn_from_future_to_bank_by_future(&mut self, pRspTransfer: *mut CThostFtdcRspTransferField) {  }
    fn on_rtn_repeal_from_bank_to_future_by_future_manual(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField) {  }
    fn on_rtn_repeal_from_future_to_bank_by_future_manual(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField) {  }
    fn on_rtn_query_bank_balance_by_future(&mut self, pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField) {  }
    fn on_err_rtn_bank_to_future_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_future_to_bank_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_repeal_bank_to_future_by_future_manual(&mut self, pReqRepeal: *mut CThostFtdcReqRepealField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_repeal_future_to_bank_by_future_manual(&mut self, pReqRepeal: *mut CThostFtdcReqRepealField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_err_rtn_query_bank_balance_by_future(&mut self, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField, pRspInfo: *mut CThostFtdcRspInfoField) {  }
    fn on_rtn_repeal_from_bank_to_future_by_future(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField) {  }
    fn on_rtn_repeal_from_future_to_bank_by_future(&mut self, pRspRepeal: *mut CThostFtdcRspRepealField) {  }
    fn on_rsp_from_bank_to_future_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_from_future_to_bank_by_future(&mut self, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_query_bank_account_money_by_future(&mut self, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rtn_open_account_by_bank(&mut self, pOpenAccount: *mut CThostFtdcOpenAccountField) {  }
    fn on_rtn_cancel_account_by_bank(&mut self, pCancelAccount: *mut CThostFtdcCancelAccountField) {  }
    fn on_rtn_change_account_by_bank(&mut self, pChangeAccount: *mut CThostFtdcChangeAccountField) {  }
}

#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnFrontConnected(trait_obj: *mut ::std::os::raw::c_void) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_front_connected()
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnFrontDisconnected(trait_obj: *mut ::std::os::raw::c_void, nReason: ::std::os::raw::c_int) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_front_disconnected(nReason)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnHeartBeatWarning(trait_obj: *mut ::std::os::raw::c_void, nTimeLapse: ::std::os::raw::c_int) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_heart_beat_warning(nTimeLapse)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspAuthenticate(trait_obj: *mut ::std::os::raw::c_void, pRspAuthenticateField: *mut CThostFtdcRspAuthenticateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_authenticate(pRspAuthenticateField, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspUserLogin(trait_obj: *mut ::std::os::raw::c_void, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspUserLogout(trait_obj: *mut ::std::os::raw::c_void, pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspUserPasswordUpdate(trait_obj: *mut ::std::os::raw::c_void, pUserPasswordUpdate: *mut CThostFtdcUserPasswordUpdateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_user_password_update(pUserPasswordUpdate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspTradingAccountPasswordUpdate(trait_obj: *mut ::std::os::raw::c_void, pTradingAccountPasswordUpdate: *mut CThostFtdcTradingAccountPasswordUpdateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_trading_account_password_update(pTradingAccountPasswordUpdate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspUserAuthMethod(trait_obj: *mut ::std::os::raw::c_void, pRspUserAuthMethod: *mut CThostFtdcRspUserAuthMethodField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_user_auth_method(pRspUserAuthMethod, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspGenUserCaptcha(trait_obj: *mut ::std::os::raw::c_void, pRspGenUserCaptcha: *mut CThostFtdcRspGenUserCaptchaField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_gen_user_captcha(pRspGenUserCaptcha, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspGenUserText(trait_obj: *mut ::std::os::raw::c_void, pRspGenUserText: *mut CThostFtdcRspGenUserTextField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_gen_user_text(pRspGenUserText, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspOrderInsert(trait_obj: *mut ::std::os::raw::c_void, pInputOrder: *mut CThostFtdcInputOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_order_insert(pInputOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspParkedOrderInsert(trait_obj: *mut ::std::os::raw::c_void, pParkedOrder: *mut CThostFtdcParkedOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_parked_order_insert(pParkedOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspParkedOrderAction(trait_obj: *mut ::std::os::raw::c_void, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspOrderAction(trait_obj: *mut ::std::os::raw::c_void, pInputOrderAction: *mut CThostFtdcInputOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_order_action(pInputOrderAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQueryMaxOrderVolume(trait_obj: *mut ::std::os::raw::c_void, pQueryMaxOrderVolume: *mut CThostFtdcQueryMaxOrderVolumeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_query_max_order_volume(pQueryMaxOrderVolume, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspSettlementInfoConfirm(trait_obj: *mut ::std::os::raw::c_void, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspRemoveParkedOrder(trait_obj: *mut ::std::os::raw::c_void, pRemoveParkedOrder: *mut CThostFtdcRemoveParkedOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_remove_parked_order(pRemoveParkedOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspRemoveParkedOrderAction(trait_obj: *mut ::std::os::raw::c_void, pRemoveParkedOrderAction: *mut CThostFtdcRemoveParkedOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_remove_parked_order_action(pRemoveParkedOrderAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspExecOrderInsert(trait_obj: *mut ::std::os::raw::c_void, pInputExecOrder: *mut CThostFtdcInputExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_exec_order_insert(pInputExecOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspExecOrderAction(trait_obj: *mut ::std::os::raw::c_void, pInputExecOrderAction: *mut CThostFtdcInputExecOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_exec_order_action(pInputExecOrderAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspForQuoteInsert(trait_obj: *mut ::std::os::raw::c_void, pInputForQuote: *mut CThostFtdcInputForQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_for_quote_insert(pInputForQuote, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQuoteInsert(trait_obj: *mut ::std::os::raw::c_void, pInputQuote: *mut CThostFtdcInputQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_quote_insert(pInputQuote, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQuoteAction(trait_obj: *mut ::std::os::raw::c_void, pInputQuoteAction: *mut CThostFtdcInputQuoteActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_quote_action(pInputQuoteAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspBatchOrderAction(trait_obj: *mut ::std::os::raw::c_void, pInputBatchOrderAction: *mut CThostFtdcInputBatchOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_batch_order_action(pInputBatchOrderAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspOptionSelfCloseInsert(trait_obj: *mut ::std::os::raw::c_void, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_option_self_close_insert(pInputOptionSelfClose, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspOptionSelfCloseAction(trait_obj: *mut ::std::os::raw::c_void, pInputOptionSelfCloseAction: *mut CThostFtdcInputOptionSelfCloseActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_option_self_close_action(pInputOptionSelfCloseAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspCombActionInsert(trait_obj: *mut ::std::os::raw::c_void, pInputCombAction: *mut CThostFtdcInputCombActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_comb_action_insert(pInputCombAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryOrder(trait_obj: *mut ::std::os::raw::c_void, pOrder: *mut CThostFtdcOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_order(pOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryTrade(trait_obj: *mut ::std::os::raw::c_void, pTrade: *mut CThostFtdcTradeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_trade(pTrade, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInvestorPosition(trait_obj: *mut ::std::os::raw::c_void, pInvestorPosition: *mut CThostFtdcInvestorPositionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_investor_position(pInvestorPosition, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryTradingAccount(trait_obj: *mut ::std::os::raw::c_void, pTradingAccount: *mut CThostFtdcTradingAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInvestor(trait_obj: *mut ::std::os::raw::c_void, pInvestor: *mut CThostFtdcInvestorField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_investor(pInvestor, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryTradingCode(trait_obj: *mut ::std::os::raw::c_void, pTradingCode: *mut CThostFtdcTradingCodeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_trading_code(pTradingCode, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInstrumentMarginRate(trait_obj: *mut ::std::os::raw::c_void, pInstrumentMarginRate: *mut CThostFtdcInstrumentMarginRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_instrument_margin_rate(pInstrumentMarginRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInstrumentCommissionRate(trait_obj: *mut ::std::os::raw::c_void, pInstrumentCommissionRate: *mut CThostFtdcInstrumentCommissionRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_instrument_commission_rate(pInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryExchange(trait_obj: *mut ::std::os::raw::c_void, pExchange: *mut CThostFtdcExchangeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_exchange(pExchange, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryProduct(trait_obj: *mut ::std::os::raw::c_void, pProduct: *mut CThostFtdcProductField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_product(pProduct, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInstrument(trait_obj: *mut ::std::os::raw::c_void, pInstrument: *mut CThostFtdcInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_instrument(pInstrument, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryDepthMarketData(trait_obj: *mut ::std::os::raw::c_void, pDepthMarketData: *mut CThostFtdcDepthMarketDataField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_depth_market_data(pDepthMarketData, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQrySettlementInfo(trait_obj: *mut ::std::os::raw::c_void, pSettlementInfo: *mut CThostFtdcSettlementInfoField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_settlement_info(pSettlementInfo, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryTransferBank(trait_obj: *mut ::std::os::raw::c_void, pTransferBank: *mut CThostFtdcTransferBankField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_transfer_bank(pTransferBank, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInvestorPositionDetail(trait_obj: *mut ::std::os::raw::c_void, pInvestorPositionDetail: *mut CThostFtdcInvestorPositionDetailField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_investor_position_detail(pInvestorPositionDetail, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryNotice(trait_obj: *mut ::std::os::raw::c_void, pNotice: *mut CThostFtdcNoticeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_notice(pNotice, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQrySettlementInfoConfirm(trait_obj: *mut ::std::os::raw::c_void, pSettlementInfoConfirm: *mut CThostFtdcSettlementInfoConfirmField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_settlement_info_confirm(pSettlementInfoConfirm, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInvestorPositionCombineDetail(trait_obj: *mut ::std::os::raw::c_void, pInvestorPositionCombineDetail: *mut CThostFtdcInvestorPositionCombineDetailField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_investor_position_combine_detail(pInvestorPositionCombineDetail, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryCFMMCTradingAccountKey(trait_obj: *mut ::std::os::raw::c_void, pCFMMCTradingAccountKey: *mut CThostFtdcCFMMCTradingAccountKeyField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_cfmmc_trading_account_key(pCFMMCTradingAccountKey, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryEWarrantOffset(trait_obj: *mut ::std::os::raw::c_void, pEWarrantOffset: *mut CThostFtdcEWarrantOffsetField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_e_warrant_offset(pEWarrantOffset, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInvestorProductGroupMargin(trait_obj: *mut ::std::os::raw::c_void, pInvestorProductGroupMargin: *mut CThostFtdcInvestorProductGroupMarginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_investor_product_group_margin(pInvestorProductGroupMargin, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryExchangeMarginRate(trait_obj: *mut ::std::os::raw::c_void, pExchangeMarginRate: *mut CThostFtdcExchangeMarginRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_exchange_margin_rate(pExchangeMarginRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryExchangeMarginRateAdjust(trait_obj: *mut ::std::os::raw::c_void, pExchangeMarginRateAdjust: *mut CThostFtdcExchangeMarginRateAdjustField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_exchange_margin_rate_adjust(pExchangeMarginRateAdjust, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryExchangeRate(trait_obj: *mut ::std::os::raw::c_void, pExchangeRate: *mut CThostFtdcExchangeRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_exchange_rate(pExchangeRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQrySecAgentACIDMap(trait_obj: *mut ::std::os::raw::c_void, pSecAgentACIDMap: *mut CThostFtdcSecAgentACIDMapField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_sec_agent_acid_map(pSecAgentACIDMap, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryProductExchRate(trait_obj: *mut ::std::os::raw::c_void, pProductExchRate: *mut CThostFtdcProductExchRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_product_exch_rate(pProductExchRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryProductGroup(trait_obj: *mut ::std::os::raw::c_void, pProductGroup: *mut CThostFtdcProductGroupField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_product_group(pProductGroup, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryMMInstrumentCommissionRate(trait_obj: *mut ::std::os::raw::c_void, pMMInstrumentCommissionRate: *mut CThostFtdcMMInstrumentCommissionRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_mm_instrument_commission_rate(pMMInstrumentCommissionRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryMMOptionInstrCommRate(trait_obj: *mut ::std::os::raw::c_void, pMMOptionInstrCommRate: *mut CThostFtdcMMOptionInstrCommRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_mm_option_instr_comm_rate(pMMOptionInstrCommRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInstrumentOrderCommRate(trait_obj: *mut ::std::os::raw::c_void, pInstrumentOrderCommRate: *mut CThostFtdcInstrumentOrderCommRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_instrument_order_comm_rate(pInstrumentOrderCommRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQrySecAgentTradingAccount(trait_obj: *mut ::std::os::raw::c_void, pTradingAccount: *mut CThostFtdcTradingAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_sec_agent_trading_account(pTradingAccount, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQrySecAgentCheckMode(trait_obj: *mut ::std::os::raw::c_void, pSecAgentCheckMode: *mut CThostFtdcSecAgentCheckModeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_sec_agent_check_mode(pSecAgentCheckMode, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQrySecAgentTradeInfo(trait_obj: *mut ::std::os::raw::c_void, pSecAgentTradeInfo: *mut CThostFtdcSecAgentTradeInfoField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_sec_agent_trade_info(pSecAgentTradeInfo, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryOptionInstrTradeCost(trait_obj: *mut ::std::os::raw::c_void, pOptionInstrTradeCost: *mut CThostFtdcOptionInstrTradeCostField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_option_instr_trade_cost(pOptionInstrTradeCost, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryOptionInstrCommRate(trait_obj: *mut ::std::os::raw::c_void, pOptionInstrCommRate: *mut CThostFtdcOptionInstrCommRateField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_option_instr_comm_rate(pOptionInstrCommRate, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryExecOrder(trait_obj: *mut ::std::os::raw::c_void, pExecOrder: *mut CThostFtdcExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_exec_order(pExecOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryForQuote(trait_obj: *mut ::std::os::raw::c_void, pForQuote: *mut CThostFtdcForQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_for_quote(pForQuote, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryQuote(trait_obj: *mut ::std::os::raw::c_void, pQuote: *mut CThostFtdcQuoteField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_quote(pQuote, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryOptionSelfClose(trait_obj: *mut ::std::os::raw::c_void, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_option_self_close(pOptionSelfClose, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryInvestUnit(trait_obj: *mut ::std::os::raw::c_void, pInvestUnit: *mut CThostFtdcInvestUnitField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_invest_unit(pInvestUnit, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryCombInstrumentGuard(trait_obj: *mut ::std::os::raw::c_void, pCombInstrumentGuard: *mut CThostFtdcCombInstrumentGuardField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_comb_instrument_guard(pCombInstrumentGuard, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryCombAction(trait_obj: *mut ::std::os::raw::c_void, pCombAction: *mut CThostFtdcCombActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_comb_action(pCombAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryTransferSerial(trait_obj: *mut ::std::os::raw::c_void, pTransferSerial: *mut CThostFtdcTransferSerialField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_transfer_serial(pTransferSerial, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryAccountregister(trait_obj: *mut ::std::os::raw::c_void, pAccountregister: *mut CThostFtdcAccountregisterField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_accountregister(pAccountregister, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspError(trait_obj: *mut ::std::os::raw::c_void, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_error(pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnOrder(trait_obj: *mut ::std::os::raw::c_void, pOrder: *mut CThostFtdcOrderField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_order(pOrder)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnTrade(trait_obj: *mut ::std::os::raw::c_void, pTrade: *mut CThostFtdcTradeField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_trade(pTrade)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnOrderInsert(trait_obj: *mut ::std::os::raw::c_void, pInputOrder: *mut CThostFtdcInputOrderField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_order_insert(pInputOrder, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnOrderAction(trait_obj: *mut ::std::os::raw::c_void, pOrderAction: *mut CThostFtdcOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_order_action(pOrderAction, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnInstrumentStatus(trait_obj: *mut ::std::os::raw::c_void, pInstrumentStatus: *mut CThostFtdcInstrumentStatusField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_instrument_status(pInstrumentStatus)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnBulletin(trait_obj: *mut ::std::os::raw::c_void, pBulletin: *mut CThostFtdcBulletinField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_bulletin(pBulletin)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnTradingNotice(trait_obj: *mut ::std::os::raw::c_void, pTradingNoticeInfo: *mut CThostFtdcTradingNoticeInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_trading_notice(pTradingNoticeInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnErrorConditionalOrder(trait_obj: *mut ::std::os::raw::c_void, pErrorConditionalOrder: *mut CThostFtdcErrorConditionalOrderField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_error_conditional_order(pErrorConditionalOrder)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnExecOrder(trait_obj: *mut ::std::os::raw::c_void, pExecOrder: *mut CThostFtdcExecOrderField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_exec_order(pExecOrder)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnExecOrderInsert(trait_obj: *mut ::std::os::raw::c_void, pInputExecOrder: *mut CThostFtdcInputExecOrderField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_exec_order_insert(pInputExecOrder, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnExecOrderAction(trait_obj: *mut ::std::os::raw::c_void, pExecOrderAction: *mut CThostFtdcExecOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_exec_order_action(pExecOrderAction, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnForQuoteInsert(trait_obj: *mut ::std::os::raw::c_void, pInputForQuote: *mut CThostFtdcInputForQuoteField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_for_quote_insert(pInputForQuote, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnQuote(trait_obj: *mut ::std::os::raw::c_void, pQuote: *mut CThostFtdcQuoteField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_quote(pQuote)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnQuoteInsert(trait_obj: *mut ::std::os::raw::c_void, pInputQuote: *mut CThostFtdcInputQuoteField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_quote_insert(pInputQuote, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnQuoteAction(trait_obj: *mut ::std::os::raw::c_void, pQuoteAction: *mut CThostFtdcQuoteActionField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_quote_action(pQuoteAction, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnForQuoteRsp(trait_obj: *mut ::std::os::raw::c_void, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_for_quote_rsp(pForQuoteRsp)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnCFMMCTradingAccountToken(trait_obj: *mut ::std::os::raw::c_void, pCFMMCTradingAccountToken: *mut CThostFtdcCFMMCTradingAccountTokenField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_cfmmc_trading_account_token(pCFMMCTradingAccountToken)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnBatchOrderAction(trait_obj: *mut ::std::os::raw::c_void, pBatchOrderAction: *mut CThostFtdcBatchOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_batch_order_action(pBatchOrderAction, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnOptionSelfClose(trait_obj: *mut ::std::os::raw::c_void, pOptionSelfClose: *mut CThostFtdcOptionSelfCloseField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_option_self_close(pOptionSelfClose)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnOptionSelfCloseInsert(trait_obj: *mut ::std::os::raw::c_void, pInputOptionSelfClose: *mut CThostFtdcInputOptionSelfCloseField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_option_self_close_insert(pInputOptionSelfClose, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnOptionSelfCloseAction(trait_obj: *mut ::std::os::raw::c_void, pOptionSelfCloseAction: *mut CThostFtdcOptionSelfCloseActionField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_option_self_close_action(pOptionSelfCloseAction, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnCombAction(trait_obj: *mut ::std::os::raw::c_void, pCombAction: *mut CThostFtdcCombActionField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_comb_action(pCombAction)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnCombActionInsert(trait_obj: *mut ::std::os::raw::c_void, pInputCombAction: *mut CThostFtdcInputCombActionField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_comb_action_insert(pInputCombAction, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryContractBank(trait_obj: *mut ::std::os::raw::c_void, pContractBank: *mut CThostFtdcContractBankField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_contract_bank(pContractBank, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryParkedOrder(trait_obj: *mut ::std::os::raw::c_void, pParkedOrder: *mut CThostFtdcParkedOrderField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_parked_order(pParkedOrder, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryParkedOrderAction(trait_obj: *mut ::std::os::raw::c_void, pParkedOrderAction: *mut CThostFtdcParkedOrderActionField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_parked_order_action(pParkedOrderAction, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryTradingNotice(trait_obj: *mut ::std::os::raw::c_void, pTradingNotice: *mut CThostFtdcTradingNoticeField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_trading_notice(pTradingNotice, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryBrokerTradingParams(trait_obj: *mut ::std::os::raw::c_void, pBrokerTradingParams: *mut CThostFtdcBrokerTradingParamsField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_broker_trading_params(pBrokerTradingParams, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQryBrokerTradingAlgos(trait_obj: *mut ::std::os::raw::c_void, pBrokerTradingAlgos: *mut CThostFtdcBrokerTradingAlgosField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_qry_broker_trading_algos(pBrokerTradingAlgos, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQueryCFMMCTradingAccountToken(trait_obj: *mut ::std::os::raw::c_void, pQueryCFMMCTradingAccountToken: *mut CThostFtdcQueryCFMMCTradingAccountTokenField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_query_cfmmc_trading_account_token(pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnFromBankToFutureByBank(trait_obj: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_from_bank_to_future_by_bank(pRspTransfer)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnFromFutureToBankByBank(trait_obj: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_from_future_to_bank_by_bank(pRspTransfer)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnRepealFromBankToFutureByBank(trait_obj: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_repeal_from_bank_to_future_by_bank(pRspRepeal)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnRepealFromFutureToBankByBank(trait_obj: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_repeal_from_future_to_bank_by_bank(pRspRepeal)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnFromBankToFutureByFuture(trait_obj: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_from_bank_to_future_by_future(pRspTransfer)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnFromFutureToBankByFuture(trait_obj: *mut ::std::os::raw::c_void, pRspTransfer: *mut CThostFtdcRspTransferField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_from_future_to_bank_by_future(pRspTransfer)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnRepealFromBankToFutureByFutureManual(trait_obj: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_repeal_from_bank_to_future_by_future_manual(pRspRepeal)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnRepealFromFutureToBankByFutureManual(trait_obj: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_repeal_from_future_to_bank_by_future_manual(pRspRepeal)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnQueryBankBalanceByFuture(trait_obj: *mut ::std::os::raw::c_void, pNotifyQueryAccount: *mut CThostFtdcNotifyQueryAccountField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_query_bank_balance_by_future(pNotifyQueryAccount)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnBankToFutureByFuture(trait_obj: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_bank_to_future_by_future(pReqTransfer, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnFutureToBankByFuture(trait_obj: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_future_to_bank_by_future(pReqTransfer, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnRepealBankToFutureByFutureManual(trait_obj: *mut ::std::os::raw::c_void, pReqRepeal: *mut CThostFtdcReqRepealField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_repeal_bank_to_future_by_future_manual(pReqRepeal, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnRepealFutureToBankByFutureManual(trait_obj: *mut ::std::os::raw::c_void, pReqRepeal: *mut CThostFtdcReqRepealField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_repeal_future_to_bank_by_future_manual(pReqRepeal, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnErrRtnQueryBankBalanceByFuture(trait_obj: *mut ::std::os::raw::c_void, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField, pRspInfo: *mut CThostFtdcRspInfoField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_err_rtn_query_bank_balance_by_future(pReqQueryAccount, pRspInfo)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnRepealFromBankToFutureByFuture(trait_obj: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_repeal_from_bank_to_future_by_future(pRspRepeal)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnRepealFromFutureToBankByFuture(trait_obj: *mut ::std::os::raw::c_void, pRspRepeal: *mut CThostFtdcRspRepealField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_repeal_from_future_to_bank_by_future(pRspRepeal)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspFromBankToFutureByFuture(trait_obj: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_from_bank_to_future_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspFromFutureToBankByFuture(trait_obj: *mut ::std::os::raw::c_void, pReqTransfer: *mut CThostFtdcReqTransferField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_from_future_to_bank_by_future(pReqTransfer, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRspQueryBankAccountMoneyByFuture(trait_obj: *mut ::std::os::raw::c_void, pReqQueryAccount: *mut CThostFtdcReqQueryAccountField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_query_bank_account_money_by_future(pReqQueryAccount, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnOpenAccountByBank(trait_obj: *mut ::std::os::raw::c_void, pOpenAccount: *mut CThostFtdcOpenAccountField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_open_account_by_bank(pOpenAccount)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnCancelAccountByBank(trait_obj: *mut ::std::os::raw::c_void, pCancelAccount: *mut CThostFtdcCancelAccountField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_cancel_account_by_bank(pCancelAccount)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_OnRtnChangeAccountByBank(trait_obj: *mut ::std::os::raw::c_void, pChangeAccount: *mut CThostFtdcChangeAccountField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcTraderSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_change_account_by_bank(pChangeAccount)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcTraderSpi_Trait_Drop(trait_obj: *mut ::std::os::raw::c_void) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcTraderSpi_Trait>;
    let _r: Box<Box<dyn Rust_CThostFtdcTraderSpi_Trait>> = unsafe { Box::from_raw(trait_obj) };
}

pub trait Rust_CThostFtdcMdSpi_Trait {
    fn on_front_connected(&mut self) {  }
    fn on_front_disconnected(&mut self, nReason: ::std::os::raw::c_int) {  }
    fn on_heart_beat_warning(&mut self, nTimeLapse: ::std::os::raw::c_int) {  }
    fn on_rsp_user_login(&mut self, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_user_logout(&mut self, pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_error(&mut self, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_sub_market_data(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_un_sub_market_data(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_sub_for_quote_rsp(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rsp_un_sub_for_quote_rsp(&mut self, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {  }
    fn on_rtn_depth_market_data(&mut self, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {  }
    fn on_rtn_for_quote_rsp(&mut self, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {  }
}

#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnFrontConnected(trait_obj: *mut ::std::os::raw::c_void) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_front_connected()
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnFrontDisconnected(trait_obj: *mut ::std::os::raw::c_void, nReason: ::std::os::raw::c_int) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_front_disconnected(nReason)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnHeartBeatWarning(trait_obj: *mut ::std::os::raw::c_void, nTimeLapse: ::std::os::raw::c_int) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_heart_beat_warning(nTimeLapse)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspUserLogin(trait_obj: *mut ::std::os::raw::c_void, pRspUserLogin: *mut CThostFtdcRspUserLoginField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_user_login(pRspUserLogin, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspUserLogout(trait_obj: *mut ::std::os::raw::c_void, pUserLogout: *mut CThostFtdcUserLogoutField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_user_logout(pUserLogout, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspError(trait_obj: *mut ::std::os::raw::c_void, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_error(pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspSubMarketData(trait_obj: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspUnSubMarketData(trait_obj: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_un_sub_market_data(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspSubForQuoteRsp(trait_obj: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRspUnSubForQuoteRsp(trait_obj: *mut ::std::os::raw::c_void, pSpecificInstrument: *mut CThostFtdcSpecificInstrumentField, pRspInfo: *mut CThostFtdcRspInfoField, nRequestID: ::std::os::raw::c_int, bIsLast: bool) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rsp_un_sub_for_quote_rsp(pSpecificInstrument, pRspInfo, nRequestID, bIsLast)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRtnDepthMarketData(trait_obj: *mut ::std::os::raw::c_void, pDepthMarketData: *mut CThostFtdcDepthMarketDataField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_depth_market_data(pDepthMarketData)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_OnRtnForQuoteRsp(trait_obj: *mut ::std::os::raw::c_void, pForQuoteRsp: *mut CThostFtdcForQuoteRspField) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let trait_obj: &mut dyn Rust_CThostFtdcMdSpi_Trait = unsafe { &mut **trait_obj };
    trait_obj.on_rtn_for_quote_rsp(pForQuoteRsp)
}
#[no_mangle]
pub extern "C" fn Rust_CThostFtdcMdSpi_Trait_Drop(trait_obj: *mut ::std::os::raw::c_void) {
    let trait_obj = trait_obj as *mut Box<dyn Rust_CThostFtdcMdSpi_Trait>;
    let _r: Box<Box<dyn Rust_CThostFtdcMdSpi_Trait>> = unsafe { Box::from_raw(trait_obj) };
}
