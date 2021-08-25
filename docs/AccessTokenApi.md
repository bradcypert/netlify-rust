# \AccessTokenApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange_ticket**](AccessTokenApi.md#exchange_ticket) | **POST** /oauth/tickets/{ticket_id}/exchange | 



## exchange_ticket

> crate::models::AccessToken exchange_ticket(ticket_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_id** | **String** |  | [required] |

### Return type

[**crate::models::AccessToken**](accessToken.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

