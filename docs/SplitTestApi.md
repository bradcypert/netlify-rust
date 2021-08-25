# \SplitTestApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_split_test**](SplitTestApi.md#create_split_test) | **POST** /sites/{site_id}/traffic_splits | 
[**disable_split_test**](SplitTestApi.md#disable_split_test) | **POST** /sites/{site_id}/traffic_splits/{split_test_id}/unpublish | 
[**enable_split_test**](SplitTestApi.md#enable_split_test) | **POST** /sites/{site_id}/traffic_splits/{split_test_id}/publish | 
[**get_split_test**](SplitTestApi.md#get_split_test) | **GET** /sites/{site_id}/traffic_splits/{split_test_id} | 
[**get_split_tests**](SplitTestApi.md#get_split_tests) | **GET** /sites/{site_id}/traffic_splits | 
[**update_split_test**](SplitTestApi.md#update_split_test) | **PUT** /sites/{site_id}/traffic_splits/{split_test_id} | 



## create_split_test

> crate::models::SplitTest create_split_test(site_id, branch_tests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**branch_tests** | [**SplitTestSetup**](SplitTestSetup.md) |  | [required] |

### Return type

[**crate::models::SplitTest**](splitTest.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_split_test

> disable_split_test(site_id, split_test_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**split_test_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_split_test

> enable_split_test(site_id, split_test_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**split_test_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_split_test

> crate::models::SplitTest get_split_test(site_id, split_test_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**split_test_id** | **String** |  | [required] |

### Return type

[**crate::models::SplitTest**](splitTest.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_split_tests

> Vec<crate::models::SplitTest> get_split_tests(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::SplitTest>**](splitTest.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_split_test

> crate::models::SplitTest update_split_test(site_id, split_test_id, branch_tests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**split_test_id** | **String** |  | [required] |
**branch_tests** | [**SplitTestSetup**](SplitTestSetup.md) |  | [required] |

### Return type

[**crate::models::SplitTest**](splitTest.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

