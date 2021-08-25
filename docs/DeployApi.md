# \DeployApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_site_deploy**](DeployApi.md#cancel_site_deploy) | **POST** /deploys/{deploy_id}/cancel | 
[**create_site_deploy**](DeployApi.md#create_site_deploy) | **POST** /sites/{site_id}/deploys | 
[**get_deploy**](DeployApi.md#get_deploy) | **GET** /deploys/{deploy_id} | 
[**get_site_deploy**](DeployApi.md#get_site_deploy) | **GET** /sites/{site_id}/deploys/{deploy_id} | 
[**list_site_deploys**](DeployApi.md#list_site_deploys) | **GET** /sites/{site_id}/deploys | 
[**lock_deploy**](DeployApi.md#lock_deploy) | **POST** /deploys/{deploy_id}/lock | 
[**restore_site_deploy**](DeployApi.md#restore_site_deploy) | **POST** /sites/{site_id}/deploys/{deploy_id}/restore | 
[**rollback_site_deploy**](DeployApi.md#rollback_site_deploy) | **PUT** /sites/{site_id}/rollback | 
[**unlock_deploy**](DeployApi.md#unlock_deploy) | **POST** /deploys/{deploy_id}/unlock | 
[**update_site_deploy**](DeployApi.md#update_site_deploy) | **PUT** /sites/{site_id}/deploys/{deploy_id} | 



## cancel_site_deploy

> crate::models::Deploy cancel_site_deploy(deploy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_site_deploy

> crate::models::Deploy create_site_deploy(site_id, deploy, title)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**deploy** | [**DeployFiles**](DeployFiles.md) |  | [required] |
**title** | Option<**String**> |  |  |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deploy

> crate::models::Deploy get_deploy(deploy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site_deploy

> crate::models::Deploy get_site_deploy(site_id, deploy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**deploy_id** | **String** |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_site_deploys

> Vec<crate::models::Deploy> list_site_deploys(site_id, page, per_page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**per_page** | Option<**i32**> |  |  |

### Return type

[**Vec<crate::models::Deploy>**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_deploy

> crate::models::Deploy lock_deploy(deploy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_site_deploy

> crate::models::Deploy restore_site_deploy(site_id, deploy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**deploy_id** | **String** |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rollback_site_deploy

> rollback_site_deploy(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_deploy

> crate::models::Deploy unlock_deploy(deploy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_id** | **String** |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_site_deploy

> crate::models::Deploy update_site_deploy(site_id, deploy_id, deploy)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**deploy_id** | **String** |  | [required] |
**deploy** | [**DeployFiles**](DeployFiles.md) |  | [required] |

### Return type

[**crate::models::Deploy**](deploy.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

