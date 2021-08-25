# \XInternalApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_plugin_run**](XInternalApi.md#create_plugin_run) | **POST** /deploys/{deploy_id}/plugin_runs | 
[**get_latest_plugin_runs**](XInternalApi.md#get_latest_plugin_runs) | **GET** /sites/{site_id}/plugin_runs/latest | 
[**update_plugin**](XInternalApi.md#update_plugin) | **PUT** /sites/{site_id}/plugins/{package} | 



## create_plugin_run

> crate::models::PluginRun create_plugin_run(deploy_id, plugin_run)


This is an internal-only endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |
**plugin_run** | Option<[**PluginRunData**](PluginRunData.md)> |  |  |

### Return type

[**crate::models::PluginRun**](pluginRun.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_plugin_runs

> Vec<crate::models::PluginRun> get_latest_plugin_runs(site_id, packages, state)


This is an internal-only endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**packages** | [**Vec<String>**](String.md) |  | [required] |
**state** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PluginRun>**](pluginRun.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plugin

> crate::models::Plugin update_plugin(site_id, package, plugin_params)


This is an internal-only endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**package** | **String** |  | [required] |
**plugin_params** | Option<[**PluginParams**](PluginParams.md)> |  |  |

### Return type

[**crate::models::Plugin**](plugin.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

