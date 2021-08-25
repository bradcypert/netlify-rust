# \ServiceInstanceApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_service_instance**](ServiceInstanceApi.md#create_service_instance) | **POST** /sites/{site_id}/services/{addon}/instances | 
[**delete_service_instance**](ServiceInstanceApi.md#delete_service_instance) | **DELETE** /sites/{site_id}/services/{addon}/instances/{instance_id} | 
[**list_service_instances_for_site**](ServiceInstanceApi.md#list_service_instances_for_site) | **GET** /sites/{site_id}/service-instances | 
[**show_service_instance**](ServiceInstanceApi.md#show_service_instance) | **GET** /sites/{site_id}/services/{addon}/instances/{instance_id} | 
[**update_service_instance**](ServiceInstanceApi.md#update_service_instance) | **PUT** /sites/{site_id}/services/{addon}/instances/{instance_id} | 



## create_service_instance

> crate::models::ServiceInstance create_service_instance(site_id, addon, config)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**addon** | **String** |  | [required] |
**config** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::ServiceInstance**](serviceInstance.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_instance

> delete_service_instance(site_id, addon, instance_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**addon** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_service_instances_for_site

> Vec<crate::models::ServiceInstance> list_service_instances_for_site(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ServiceInstance>**](serviceInstance.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_service_instance

> crate::models::ServiceInstance show_service_instance(site_id, addon, instance_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**addon** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |

### Return type

[**crate::models::ServiceInstance**](serviceInstance.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_instance

> update_service_instance(site_id, addon, instance_id, config)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**addon** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |
**config** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

