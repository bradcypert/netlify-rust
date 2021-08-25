# \BuildHookApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_site_build_hook**](BuildHookApi.md#create_site_build_hook) | **POST** /sites/{site_id}/build_hooks | 
[**delete_site_build_hook**](BuildHookApi.md#delete_site_build_hook) | **DELETE** /sites/{site_id}/build_hooks/{id} | 
[**get_site_build_hook**](BuildHookApi.md#get_site_build_hook) | **GET** /sites/{site_id}/build_hooks/{id} | 
[**list_site_build_hooks**](BuildHookApi.md#list_site_build_hooks) | **GET** /sites/{site_id}/build_hooks | 
[**update_site_build_hook**](BuildHookApi.md#update_site_build_hook) | **PUT** /sites/{site_id}/build_hooks/{id} | 



## create_site_build_hook

> crate::models::BuildHook create_site_build_hook(site_id, build_hook)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**build_hook** | [**BuildHookSetup**](BuildHookSetup.md) |  | [required] |

### Return type

[**crate::models::BuildHook**](buildHook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_site_build_hook

> delete_site_build_hook(site_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_build_hook

> crate::models::BuildHook get_site_build_hook(site_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::BuildHook**](buildHook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_build_hooks

> Vec<crate::models::BuildHook> list_site_build_hooks(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::BuildHook>**](buildHook.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site_build_hook

> update_site_build_hook(site_id, id, build_hook)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**build_hook** | [**BuildHookSetup**](BuildHookSetup.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

