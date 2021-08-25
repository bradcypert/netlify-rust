# \BuildLogMsgApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_site_build_log**](BuildLogMsgApi.md#update_site_build_log) | **POST** /builds/{build_id}/log | 



## update_site_build_log

> update_site_build_log(build_id, msg)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_id** | **String** |  | [required] |
**msg** | [**BuildLogMsg**](BuildLogMsg.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

