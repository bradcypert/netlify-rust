# \SniCertificateApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**provision_site_tls_certificate**](SniCertificateApi.md#provision_site_tls_certificate) | **POST** /sites/{site_id}/ssl | 
[**show_site_tls_certificate**](SniCertificateApi.md#show_site_tls_certificate) | **GET** /sites/{site_id}/ssl | 



## provision_site_tls_certificate

> crate::models::SniCertificate provision_site_tls_certificate(site_id, certificate, key, ca_certificates)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |
**certificate** | Option<**String**> |  |  |
**key** | Option<**String**> |  |  |
**ca_certificates** | Option<**String**> |  |  |

### Return type

[**crate::models::SniCertificate**](sniCertificate.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## show_site_tls_certificate

> crate::models::SniCertificate show_site_tls_certificate(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**crate::models::SniCertificate**](sniCertificate.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

