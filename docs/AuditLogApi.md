# \AuditLogApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_account_audit_events**](AuditLogApi.md#list_account_audit_events) | **GET** /accounts/{account_id}/audit | 



## list_account_audit_events

> Vec<crate::models::AuditLog> list_account_audit_events(account_id, query, log_type, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**query** | Option<**String**> |  |  |
**log_type** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::AuditLog>**](auditLog.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

