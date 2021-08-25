# \BuildApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_site_build**](BuildApi.md#create_site_build) | **POST** /sites/{site_id}/builds | 
[**get_account_build_status**](BuildApi.md#get_account_build_status) | **GET** /{account_id}/builds/status | 
[**get_site_build**](BuildApi.md#get_site_build) | **GET** /builds/{build_id} | 
[**list_site_builds**](BuildApi.md#list_site_builds) | **GET** /sites/{site_id}/builds | 
[**notify_build_start**](BuildApi.md#notify_build_start) | **POST** /builds/{build_id}/start | 



## create_site_build

> crate::models::Build create_site_build(site_id, build)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**build** | Option<[**BuildSetup**](BuildSetup.md)> |  |  |

### Return type

[**crate::models::Build**](build.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_build_status

> Vec<crate::models::BuildStatus> get_account_build_status(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::BuildStatus>**](buildStatus.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_build

> crate::models::Build get_site_build(build_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_id** | **String** |  | [required] |

### Return type

[**crate::models::Build**](build.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_builds

> Vec<crate::models::Build> list_site_builds(site_id, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Build>**](build.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notify_build_start

> notify_build_start(build_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

