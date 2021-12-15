use crate::{client::Client, end_points::*};
use reqwest::Body;
use serde::{Deserialize, Serialize};
impl Client {
    pub async fn get_<T, V>(&self, query: Option<&T>, body: Option<V>) -> GetResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::Get(), query, body).await.unwrap()
    }
    pub async fn get_app<T, V>(&self, query: Option<&T>, body: Option<V>) -> GetAppResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetApp(), query, body).await.unwrap()
    }
    pub async fn get_app_hook_config<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetAppHookConfigResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetAppHookConfig(), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_app_hook_config<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchAppHookConfigResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchAppHookConfig(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_app_hook_deliveries_delivery_id<T, V>(
        &self,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetAppHookDeliveriesdeliveryIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAppHookDeliveriesdeliveryId(delivery_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_app_installations_installation_id<T, V>(
        &self,
        installation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetAppInstallationsinstallationIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAppInstallationsinstallationId(installation_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_applications_grants_grant_id<T, V>(
        &self,
        grant_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetApplicationsGrantsgrantIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetApplicationsGrantsgrantId(grant_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_applications_client_id_token<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostApplicationsclientIdTokenResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostApplicationsclientIdToken(client_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_applications_client_id_token<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchApplicationsclientIdTokenResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchApplicationsclientIdToken(client_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_applications_client_id_token_scoped<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostApplicationsclientIdTokenScopedResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostApplicationsclientIdTokenScoped(client_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_apps_app_slug<T, V>(
        &self,
        app_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetAppsappSlugResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetAppsappSlug(app_slug), query, body)
            .await
            .unwrap()
    }
    pub async fn put_authorizations_clients_client_id<T, V>(
        &self,
        client_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutAuthorizationsClientsclientIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutAuthorizationsClientsclientId(client_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_authorizations_clients_client_id_fingerprint<T, V>(
        &self,
        client_id: String,
        fingerprint: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutAuthorizationsClientsclientIdfingerprintResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutAuthorizationsClientsclientIdfingerprint(client_id, fingerprint),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_authorizations_authorization_id<T, V>(
        &self,
        authorization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetAuthorizationsauthorizationIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetAuthorizationsauthorizationId(authorization_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_authorizations_authorization_id<T, V>(
        &self,
        authorization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchAuthorizationsauthorizationIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchAuthorizationsauthorizationId(authorization_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_codes_of_conduct_key<T, V>(
        &self,
        key: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetCodesOfConductkeyResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetCodesOfConductkey(key), query, body)
            .await
            .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_permissions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsPermissionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissions(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_permissions_organizations<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsPermissionsOrganizationsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissionsOrganizations(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_permissions_selected_actions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsPermissionsSelectedActionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsPermissionsSelectedActions(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnerGroupsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroups(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_enterprises_enterprise_actions_runner_groups_runner_group_id<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchEnterprisesenterpriseActionsRunnerGroupsrunnerGroupId(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id_organizations<
        T,
        V,
    >(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizationsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdOrganizations(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runner_groups_runner_group_id_runners<T, V>(
        &self,
        enterprise: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunnersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnerGroupsrunnerGroupIdRunners(
                enterprise,
                runner_group_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runners<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunners(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runners_runner_id<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnersrunnerIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerId(enterprise, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutEnterprisesenterpriseActionsRunnersrunnerIdLabels(enterprise, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_enterprises_enterprise_actions_runners_runner_id_labels<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabels(
                enterprise, runner_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_enterprises_enterprise_actions_runners_runner_id_labels_name<T, V>(
        &self,
        enterprise: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsnameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteEnterprisesenterpriseActionsRunnersrunnerIdLabelsname(
                enterprise, runner_id, name,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_settings_billing_actions<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseSettingsBillingActionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingActions(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_settings_billing_advanced_security<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseSettingsBillingAdvancedSecurityResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingAdvancedSecurity(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_settings_billing_packages<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseSettingsBillingPackagesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingPackages(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_enterprises_enterprise_settings_billing_shared_storage<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetEnterprisesenterpriseSettingsBillingSharedStorageResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetEnterprisesenterpriseSettingsBillingSharedStorage(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_feeds<T, V>(&self, query: Option<&T>, body: Option<V>) -> GetFeedsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetFeeds(), query, body).await.unwrap()
    }
    pub async fn get_gists_gist_id<T, V>(
        &self,
        gist_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetGistsgistIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGistsgistId(gist_id), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_gists_gist_id<T, V>(
        &self,
        gist_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchGistsgistIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchGistsgistId(gist_id), query, body)
            .await
            .unwrap()
    }
    pub async fn get_gists_gist_id_comments_comment_id<T, V>(
        &self,
        gist_id: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetGistsgistIdCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetGistsgistIdCommentscommentId(gist_id, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_gists_gist_id_comments_comment_id<T, V>(
        &self,
        gist_id: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchGistsgistIdCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchGistsgistIdCommentscommentId(gist_id, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_gists_gist_id_sha<T, V>(
        &self,
        gist_id: String,
        sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetGistsgistIdshaResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGistsgistIdsha(gist_id, sha), query, body)
            .await
            .unwrap()
    }
    pub async fn get_gitignore_templates_name<T, V>(
        &self,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetGitignoreTemplatesnameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetGitignoreTemplatesname(name), query, body)
            .await
            .unwrap()
    }
    pub async fn get_installation_repositories<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetInstallationRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetInstallationRepositories(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_licenses_license<T, V>(
        &self,
        license: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetLicenseslicenseResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetLicenseslicense(license), query, body)
            .await
            .unwrap()
    }
    pub async fn get_marketplace_listing_accounts_account_id<T, V>(
        &self,
        account_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetMarketplaceListingAccountsaccountIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetMarketplaceListingAccountsaccountId(account_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_marketplace_listing_stubbed_accounts_account_id<T, V>(
        &self,
        account_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetMarketplaceListingStubbedAccountsaccountIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetMarketplaceListingStubbedAccountsaccountId(account_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_meta<T, V>(&self, query: Option<&T>, body: Option<V>) -> GetMetaResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetMeta(), query, body).await.unwrap()
    }
    pub async fn get_notifications_threads_thread_id<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetNotificationsThreadsthreadIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetNotificationsThreadsthreadId(thread_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_notifications_threads_thread_id_subscription<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetNotificationsThreadsthreadIdSubscriptionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetNotificationsThreadsthreadIdSubscription(thread_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_notifications_threads_thread_id_subscription<T, V>(
        &self,
        thread_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutNotificationsThreadsthreadIdSubscriptionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutNotificationsThreadsthreadIdSubscription(thread_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_organizations_organization_id_custom_roles<T, V>(
        &self,
        organization_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrganizationsorganizationIdCustomRolesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrganizationsorganizationIdCustomRoles(organization_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorg(org), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchOrgsorg(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_actions_permissions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsPermissionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsPermissions(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_actions_permissions_repositories<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsPermissionsRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsPermissionsRepositories(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_permissions_selected_actions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsPermissionsSelectedActionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsPermissionsSelectedActions(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_runner_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnerGroupsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsRunnerGroups(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnerGroupsrunnerGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupId(org, runner_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_orgs_org_actions_runner_groups_runner_group_id<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgActionsRunnerGroupsrunnerGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgActionsRunnerGroupsrunnerGroupId(org, runner_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id_repositories<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRepositories(org, runner_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_runner_groups_runner_group_id_runners<T, V>(
        &self,
        org: String,
        runner_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunnersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnerGroupsrunnerGroupIdRunners(org, runner_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_runners<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsRunners(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_actions_runners_runner_id<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnersrunnerIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnersrunnerId(org, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostOrgsorgActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutOrgsorgActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_orgs_org_actions_runners_runner_id_labels<T, V>(
        &self,
        org: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteOrgsorgActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabels(org, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_orgs_org_actions_runners_runner_id_labels_name<T, V>(
        &self,
        org: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteOrgsorgActionsRunnersrunnerIdLabelsnameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteOrgsorgActionsRunnersrunnerIdLabelsname(org, runner_id, name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_secrets<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsSecretsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgActionsSecrets(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_actions_secrets_public_key<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsSecretsPublicKeyResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretsPublicKey(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_secrets_secret_name<T, V>(
        &self,
        org: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsSecretssecretNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretssecretName(org, secret_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_actions_secrets_secret_name_repositories<T, V>(
        &self,
        org: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgActionsSecretssecretNameRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgActionsSecretssecretNameRepositories(org, secret_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_external_group_group_id<T, V>(
        &self,
        org: String,
        group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgExternalGroupgroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgExternalGroupgroupId(org, group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_external_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgExternalGroupsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgExternalGroups(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_hooks_hook_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgHookshookIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgHookshookId(org, hook_id), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_orgs_org_hooks_hook_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgHookshookIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgHookshookId(org, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_hooks_hook_id_config<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgHookshookIdConfigResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgHookshookIdConfig(org, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_orgs_org_hooks_hook_id_config<T, V>(
        &self,
        org: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgHookshookIdConfigResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgHookshookIdConfig(org, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_hooks_hook_id_deliveries_delivery_id<T, V>(
        &self,
        org: String,
        hook_id: String,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgHookshookIdDeliveriesdeliveryIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgHookshookIdDeliveriesdeliveryId(org, hook_id, delivery_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_installation<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgInstallationResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgInstallation(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_installations<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgInstallationsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgInstallations(org), query, body)
            .await
            .unwrap()
    }
    pub async fn put_orgs_org_interaction_limits<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutOrgsorgInteractionLimitsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutOrgsorgInteractionLimits(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_memberships_username<T, V>(
        &self,
        org: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgMembershipsusernameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgMembershipsusername(org, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_orgs_org_memberships_username<T, V>(
        &self,
        org: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutOrgsorgMembershipsusernameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgMembershipsusername(org, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_migrations_migration_id<T, V>(
        &self,
        org: String,
        migration_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgMigrationsmigrationIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgMigrationsmigrationId(org, migration_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_packages_package_type_package_name<T, V>(
        &self,
        org: String,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgPackagespackageTypepackageNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgPackagespackageTypepackageName(org, package_type, package_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_packages_package_type_package_name_versions_package_version_id<T, V>(
        &self,
        org: String,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgPackagespackageTypepackageNameVersionspackageVersionId(
                org,
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_settings_billing_actions<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgSettingsBillingActionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingActions(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_settings_billing_advanced_security<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgSettingsBillingAdvancedSecurityResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingAdvancedSecurity(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_settings_billing_packages<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgSettingsBillingPackagesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingPackages(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_settings_billing_shared_storage<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgSettingsBillingSharedStorageResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgSettingsBillingSharedStorage(org),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_team_sync_groups<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamSyncGroupsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetOrgsorgTeamSyncGroups(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlug(org, team_slug),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug_discussions_discussion_number<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_orgs_org_teams_team_slug_discussions_discussion_number<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumber(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
                org,
                team_slug,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumber(
                org,
                team_slug,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_orgs_org_teams_team_slug_discussions_discussion_number_comments_comment_number_reactions<
        T,
        V,
    >(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberCommentscommentNumberReactions(org, team_slug, discussion_number, comment_number), query, body).await.unwrap()
    }
    pub async fn post_orgs_org_teams_team_slug_discussions_discussion_number_reactions<T, V>(
        &self,
        org: String,
        team_slug: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostOrgsorgTeamsteamSlugDiscussionsdiscussionNumberReactions(
                org,
                team_slug,
                discussion_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_orgs_org_teams_team_slug_external_groups<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgTeamsteamSlugExternalGroupsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugExternalGroups(org, team_slug),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug_memberships_username<T, V>(
        &self,
        org: String,
        team_slug: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugMembershipsusernameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugMembershipsusername(org, team_slug, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_orgs_org_teams_team_slug_memberships_username<T, V>(
        &self,
        org: String,
        team_slug: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutOrgsorgTeamsteamSlugMembershipsusernameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutOrgsorgTeamsteamSlugMembershipsusername(org, team_slug, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug_projects_project_id<T, V>(
        &self,
        org: String,
        team_slug: String,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugProjectsprojectIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugProjectsprojectId(org, team_slug, project_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug_repos_owner_repo<T, V>(
        &self,
        org: String,
        team_slug: String,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugReposownerrepoResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugReposownerrepo(org, team_slug, owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_orgs_org_teams_team_slug_team_sync_group_mappings<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetOrgsorgTeamsteamSlugTeamSyncGroupMappings(org, team_slug),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_orgs_org_teams_team_slug_team_sync_group_mappings<T, V>(
        &self,
        org: String,
        team_slug: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchOrgsorgTeamsteamSlugTeamSyncGroupMappingsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchOrgsorgTeamsteamSlugTeamSyncGroupMappings(org, team_slug),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_projects_columns_cards_card_id<T, V>(
        &self,
        card_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetProjectsColumnsCardscardIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsColumnsCardscardId(card_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_projects_columns_cards_card_id<T, V>(
        &self,
        card_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchProjectsColumnsCardscardIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchProjectsColumnsCardscardId(card_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_projects_columns_column_id<T, V>(
        &self,
        column_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetProjectsColumnscolumnIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsColumnscolumnId(column_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_projects_columns_column_id<T, V>(
        &self,
        column_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchProjectsColumnscolumnIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchProjectsColumnscolumnId(column_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_projects_project_id<T, V>(
        &self,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetProjectsprojectIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetProjectsprojectId(project_id), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_projects_project_id<T, V>(
        &self,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchProjectsprojectIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchProjectsprojectId(project_id), query, body)
            .await
            .unwrap()
    }
    pub async fn get_projects_project_id_collaborators_username_permission<T, V>(
        &self,
        project_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetProjectsprojectIdCollaboratorsusernamePermissionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetProjectsprojectIdCollaboratorsusernamePermission(project_id, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_rate_limit<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetRateLimitResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetRateLimit(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_repos_owner_repo<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepo(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_repos_owner_repo<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchReposownerrepo(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_artifacts<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsArtifactsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsArtifacts(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_artifacts_artifact_id<T, V>(
        &self,
        owner: String,
        repo: String,
        artifact_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsArtifactsartifactIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsArtifactsartifactId(owner, repo, artifact_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_jobs_job_id<T, V>(
        &self,
        owner: String,
        repo: String,
        job_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsJobsjobIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsJobsjobId(owner, repo, job_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_permissions<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsPermissionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsPermissions(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_permissions_selected_actions<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsPermissionsSelectedActionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsPermissionsSelectedActions(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runners<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunnersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunners(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runners_runner_id<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunnersrunnerIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunnersrunnerId(owner, repo, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_actions_runners_runner_id_labels<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoActionsRunnersrunnerIdLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabels(owner, repo, runner_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_actions_runners_runner_id_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        runner_id: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoActionsRunnersrunnerIdLabelsnameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoActionsRunnersrunnerIdLabelsname(
                owner, repo, runner_id, name,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRuns(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsrunIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunId(owner, repo, run_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_artifacts<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsrunIdArtifactsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdArtifacts(owner, repo, run_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_attempts_attempt_number<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        attempt_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumber(
                owner,
                repo,
                run_id,
                attempt_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_attempts_attempt_number_jobs<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        attempt_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdAttemptsattemptNumberJobs(
                owner,
                repo,
                run_id,
                attempt_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_jobs<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsrunIdJobsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdJobs(owner, repo, run_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_runs_run_id_timing<T, V>(
        &self,
        owner: String,
        repo: String,
        run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsRunsrunIdTimingResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsRunsrunIdTiming(owner, repo, run_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_secrets<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsSecretsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecrets(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_secrets_public_key<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsSecretsPublicKeyResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecretsPublicKey(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_secrets_secret_name<T, V>(
        &self,
        owner: String,
        repo: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsSecretssecretNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsSecretssecretName(owner, repo, secret_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_workflows<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsWorkflowsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflows(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsWorkflowsworkflowIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowId(owner, repo, workflow_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsWorkflowsworkflowIdRunsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdRuns(owner, repo, workflow_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_actions_workflows_workflow_id_timing<T, V>(
        &self,
        owner: String,
        repo: String,
        workflow_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoActionsWorkflowsworkflowIdTimingResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoActionsWorkflowsworkflowIdTiming(owner, repo, workflow_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_autolinks_autolink_id<T, V>(
        &self,
        owner: String,
        repo: String,
        autolink_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoAutolinksautolinkIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoAutolinksautolinkId(owner, repo, autolink_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranch(owner, repo, branch),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch_protection<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchProtectionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtection(owner, repo, branch),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_branches_branch_protection<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoBranchesbranchProtectionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoBranchesbranchProtection(owner, repo, branch),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_enforce_admins<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchProtectionEnforceAdminsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionEnforceAdmins(owner, repo, branch),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_branches_branch_protection_enforce_admins<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoBranchesbranchProtectionEnforceAdminsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoBranchesbranchProtectionEnforceAdmins(owner, repo, branch),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_required_pull_request_reviews<
        T,
        V,
    >(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_branches_branch_protection_required_pull_request_reviews<
        T,
        V,
    >(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviewsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredPullRequestReviews(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_required_signatures<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredSignatures(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_branches_branch_protection_required_signatures<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoBranchesbranchProtectionRequiredSignaturesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoBranchesbranchProtectionRequiredSignatures(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_required_status_checks<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRequiredStatusChecks(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_branches_branch_protection_required_status_checks<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecksResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoBranchesbranchProtectionRequiredStatusChecks(
                owner, repo, branch,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_branches_branch_protection_restrictions<T, V>(
        &self,
        owner: String,
        repo: String,
        branch: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoBranchesbranchProtectionRestrictionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoBranchesbranchProtectionRestrictions(owner, repo, branch),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_check_runs_check_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCheckRunscheckRunIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckRunscheckRunId(owner, repo, check_run_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_check_runs_check_run_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_run_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoCheckRunscheckRunIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCheckRunscheckRunId(owner, repo, check_run_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_check_suites<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoCheckSuitesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoCheckSuites(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_check_suites_preferences<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoCheckSuitesPreferencesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCheckSuitesPreferences(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_check_suites_check_suite_id<T, V>(
        &self,
        owner: String,
        repo: String,
        check_suite_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCheckSuitescheckSuiteIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckSuitescheckSuiteId(owner, repo, check_suite_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_check_suites_check_suite_id_check_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        check_suite_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCheckSuitescheckSuiteIdCheckRunsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCheckSuitescheckSuiteIdCheckRuns(
                owner,
                repo,
                check_suite_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_code_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCodeScanningAlertsalertNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_code_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoCodeScanningAlertsalertNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCodeScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_code_scanning_analyses_analysis_id<T, V>(
        &self,
        owner: String,
        repo: String,
        analysis_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCodeScanningAnalysesanalysisIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningAnalysesanalysisId(owner, repo, analysis_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_code_scanning_analyses_analysis_id<T, V>(
        &self,
        owner: String,
        repo: String,
        analysis_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoCodeScanningAnalysesanalysisIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoCodeScanningAnalysesanalysisId(owner, repo, analysis_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_code_scanning_sarifs_sarif_id<T, V>(
        &self,
        owner: String,
        repo: String,
        sarif_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCodeScanningSarifssarifIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodeScanningSarifssarifId(owner, repo, sarif_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_codespaces<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCodespacesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodespaces(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_codespaces_machines<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCodespacesMachinesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCodespacesMachines(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_collaborators_username_permission<T, V>(
        &self,
        owner: String,
        repo: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCollaboratorsusernamePermissionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCollaboratorsusernamePermission(owner, repo, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoCommentscommentIdReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_commits_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCommitsrefResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsref(owner, repo, aref),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_commits_aref_check_runs<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCommitsrefCheckRunsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefCheckRuns(owner, repo, aref),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_commits_aref_check_suites<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCommitsrefCheckSuitesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefCheckSuites(owner, repo, aref),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_commits_aref_status<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCommitsrefStatusResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommitsrefStatus(owner, repo, aref),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_community_profile<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoCommunityProfileResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoCommunityProfile(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_compare_basehead<T, V>(
        &self,
        owner: String,
        repo: String,
        basehead: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoComparebaseheadResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoComparebasehead(owner, repo, basehead),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_contents_path<T, V>(
        &self,
        owner: String,
        repo: String,
        path: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoContentspathResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoContentspath(owner, repo, path),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_contents_path<T, V>(
        &self,
        owner: String,
        repo: String,
        path: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoContentspathResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoContentspath(owner, repo, path),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_deployments_deployment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        deployment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoDeploymentsdeploymentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoDeploymentsdeploymentId(owner, repo, deployment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_deployments_deployment_id_statuses_status_id<T, V>(
        &self,
        owner: String,
        repo: String,
        deployment_id: String,
        status_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoDeploymentsdeploymentIdStatusesstatusIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoDeploymentsdeploymentIdStatusesstatusId(
                owner,
                repo,
                deployment_id,
                status_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_environments<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoEnvironmentsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoEnvironments(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_environments_environment_name<T, V>(
        &self,
        owner: String,
        repo: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoEnvironmentsenvironmentNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoEnvironmentsenvironmentName(owner, repo, environment_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_environments_environment_name<T, V>(
        &self,
        owner: String,
        repo: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoEnvironmentsenvironmentNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoEnvironmentsenvironmentName(owner, repo, environment_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_git_blobs_file_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        file_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoGitBlobsfileShaResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitBlobsfileSha(owner, repo, file_sha),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_git_commits_commit_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        commit_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoGitCommitscommitShaResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitCommitscommitSha(owner, repo, commit_sha),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_git_ref_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoGitRefrefResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitRefref(owner, repo, aref),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_git_refs_aref<T, V>(
        &self,
        owner: String,
        repo: String,
        aref: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoGitRefsrefResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoGitRefsref(owner, repo, aref),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_git_tags_tag_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        tag_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoGitTagstagShaResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitTagstagSha(owner, repo, tag_sha),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_git_trees_tree_sha<T, V>(
        &self,
        owner: String,
        repo: String,
        tree_sha: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoGitTreestreeShaResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoGitTreestreeSha(owner, repo, tree_sha),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_hooks_hook_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoHookshookIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookId(owner, repo, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_hooks_hook_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoHookshookIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoHookshookId(owner, repo, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_hooks_hook_id_config<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoHookshookIdConfigResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookIdConfig(owner, repo, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_hooks_hook_id_config<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoHookshookIdConfigResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoHookshookIdConfig(owner, repo, hook_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_hooks_hook_id_deliveries_delivery_id<T, V>(
        &self,
        owner: String,
        repo: String,
        hook_id: String,
        delivery_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoHookshookIdDeliveriesdeliveryIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoHookshookIdDeliveriesdeliveryId(
                owner,
                repo,
                hook_id,
                delivery_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_import<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoImportResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoImport(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_repos_owner_repo_import<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoImportResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImport(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_import_authors_author_id<T, V>(
        &self,
        owner: String,
        repo: String,
        author_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoImportAuthorsauthorIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImportAuthorsauthorId(owner, repo, author_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_import_lfs<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoImportLfsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoImportLfs(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_installation<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoInstallationResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoInstallation(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_interaction_limits<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoInteractionLimitsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoInteractionLimits(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_invitations_invitation_id<T, V>(
        &self,
        owner: String,
        repo: String,
        invitation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoInvitationsinvitationIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoInvitationsinvitationId(owner, repo, invitation_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_issues_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoIssuesCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_issues_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoIssuesCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoIssuesCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_issues_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoIssuesCommentscommentIdReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoIssuesCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_issues_events_event_id<T, V>(
        &self,
        owner: String,
        repo: String,
        event_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoIssuesEventseventIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesEventseventId(owner, repo, event_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_issues_issue_number<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoIssuesissueNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoIssuesissueNumber(owner, repo, issue_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_issues_issue_number<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoIssuesissueNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoIssuesissueNumber(owner, repo, issue_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_issues_issue_number_assignees<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoIssuesissueNumberAssigneesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoIssuesissueNumberAssignees(owner, repo, issue_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_issues_issue_number_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        issue_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoIssuesissueNumberReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoIssuesissueNumberReactions(owner, repo, issue_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_keys_key_id<T, V>(
        &self,
        owner: String,
        repo: String,
        key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoKeyskeyIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoKeyskeyId(owner, repo, key_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoLabelsnameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoLabelsname(owner, repo, name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_labels_name<T, V>(
        &self,
        owner: String,
        repo: String,
        name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoLabelsnameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoLabelsname(owner, repo, name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_license<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoLicenseResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoLicense(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_merge_upstream<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoMergeUpstreamResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoMergeUpstream(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_milestones_milestone_number<T, V>(
        &self,
        owner: String,
        repo: String,
        milestone_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoMilestonesmilestoneNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoMilestonesmilestoneNumber(owner, repo, milestone_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_milestones_milestone_number<T, V>(
        &self,
        owner: String,
        repo: String,
        milestone_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoMilestonesmilestoneNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoMilestonesmilestoneNumber(owner, repo, milestone_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pages<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPagesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoPages(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn get_repos_owner_repo_pages_builds_latest<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPagesBuildsLatestResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesBuildsLatest(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pages_builds_build_id<T, V>(
        &self,
        owner: String,
        repo: String,
        build_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPagesBuildsbuildIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesBuildsbuildId(owner, repo, build_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pages_health<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPagesHealthResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPagesHealth(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pulls_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPullsCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullsCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_pulls_comments_comment_id<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoPullsCommentscommentIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoPullsCommentscommentId(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_pulls_comments_comment_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        comment_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoPullsCommentscommentIdReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullsCommentscommentIdReactions(owner, repo, comment_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pulls_pull_number<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPullspullNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumber(owner, repo, pull_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_pulls_pull_number<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoPullspullNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoPullspullNumber(owner, repo, pull_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_pulls_pull_number_merge<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoPullspullNumberMergeResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberMerge(owner, repo, pull_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pulls_pull_number_requested_reviewers<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPullspullNumberRequestedReviewersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumberRequestedReviewers(owner, repo, pull_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_pulls_pull_number_requested_reviewers<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoPullspullNumberRequestedReviewersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoPullspullNumberRequestedReviewers(
                owner,
                repo,
                pull_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_pulls_pull_number_reviews<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoPullspullNumberReviewsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullspullNumberReviews(owner, repo, pull_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoPullspullNumberReviewsreviewIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoPullspullNumberReviewsreviewIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn delete_repos_owner_repo_pulls_pull_number_reviews_review_id<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> DeleteReposownerrepoPullspullNumberReviewsreviewIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::DeleteReposownerrepoPullspullNumberReviewsreviewId(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_pulls_pull_number_reviews_review_id_dismissals<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoPullspullNumberReviewsreviewIdDismissalsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoPullspullNumberReviewsreviewIdDismissals(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_pulls_pull_number_reviews_review_id_events<T, V>(
        &self,
        owner: String,
        repo: String,
        pull_number: String,
        review_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoPullspullNumberReviewsreviewIdEventsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoPullspullNumberReviewsreviewIdEvents(
                owner,
                repo,
                pull_number,
                review_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_readme<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoReadmeResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoReadme(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn get_repos_owner_repo_readme_dir<T, V>(
        &self,
        owner: String,
        repo: String,
        dir: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoReadmedirResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReadmedir(owner, repo, dir),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_releases_assets_asset_id<T, V>(
        &self,
        owner: String,
        repo: String,
        asset_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoReleasesAssetsassetIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesAssetsassetId(owner, repo, asset_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_releases_assets_asset_id<T, V>(
        &self,
        owner: String,
        repo: String,
        asset_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoReleasesAssetsassetIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoReleasesAssetsassetId(owner, repo, asset_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_releases_generate_notes<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoReleasesGenerateNotesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoReleasesGenerateNotes(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_releases_latest<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoReleasesLatestResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesLatest(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_releases_tags_tag<T, V>(
        &self,
        owner: String,
        repo: String,
        tag: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoReleasesTagstagResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesTagstag(owner, repo, tag),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_releases_release_id<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoReleasesreleaseIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoReleasesreleaseId(owner, repo, release_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_releases_release_id<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoReleasesreleaseIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoReleasesreleaseId(owner, repo, release_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_repos_owner_repo_releases_release_id_reactions<T, V>(
        &self,
        owner: String,
        repo: String,
        release_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostReposownerrepoReleasesreleaseIdReactionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostReposownerrepoReleasesreleaseIdReactions(owner, repo, release_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_secret_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoSecretScanningAlertsalertNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoSecretScanningAlertsalertNumber(owner, repo, alert_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_repos_owner_repo_secret_scanning_alerts_alert_number<T, V>(
        &self,
        owner: String,
        repo: String,
        alert_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchReposownerrepoSecretScanningAlertsalertNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchReposownerrepoSecretScanningAlertsalertNumber(
                owner,
                repo,
                alert_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_stats_participation<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoStatsParticipationResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoStatsParticipation(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_subscription<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoSubscriptionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoSubscription(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_repos_owner_repo_subscription<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoSubscriptionResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutReposownerrepoSubscription(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_topics<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoTopicsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetReposownerrepoTopics(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn put_repos_owner_repo_topics<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutReposownerrepoTopicsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutReposownerrepoTopics(owner, repo), query, body)
            .await
            .unwrap()
    }
    pub async fn get_repos_owner_repo_traffic_clones<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoTrafficClonesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoTrafficClones(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repos_owner_repo_traffic_views<T, V>(
        &self,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetReposownerrepoTrafficViewsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetReposownerrepoTrafficViews(owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repositories_repository_id_environments_environment_name_secrets<T, V>(
        &self,
        repository_id: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecrets(
                repository_id,
                environment_name,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repositories_repository_id_environments_environment_name_secrets_public_key<
        T,
        V,
    >(
        &self,
        repository_id: String,
        environment_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKeyResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretsPublicKey(
                repository_id,
                environment_name,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_repositories_repository_id_environments_environment_name_secrets_secret_name<
        T,
        V,
    >(
        &self,
        repository_id: String,
        environment_name: String,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetRepositoriesrepositoryIdEnvironmentsenvironmentNameSecretssecretName(
                repository_id,
                environment_name,
                secret_name,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_scim_v2_enterprises_enterprise_Groups<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetScimV2EnterprisesenterpriseGroupsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseGroups(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetScimV2EnterprisesenterpriseGroupsscimGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutScimV2EnterprisesenterpriseGroupsscimGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_scim_v2_enterprises_enterprise_Groups_scim_group_id<T, V>(
        &self,
        enterprise: String,
        scim_group_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchScimV2EnterprisesenterpriseGroupsscimGroupIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchScimV2EnterprisesenterpriseGroupsscimGroupId(enterprise, scim_group_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_scim_v2_enterprises_enterprise_Users<T, V>(
        &self,
        enterprise: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetScimV2EnterprisesenterpriseUsersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseUsers(enterprise),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetScimV2EnterprisesenterpriseUsersscimUserIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutScimV2EnterprisesenterpriseUsersscimUserIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_scim_v2_enterprises_enterprise_Users_scim_user_id<T, V>(
        &self,
        enterprise: String,
        scim_user_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchScimV2EnterprisesenterpriseUsersscimUserIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchScimV2EnterprisesenterpriseUsersscimUserId(enterprise, scim_user_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_search_code<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchCodeResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchCode(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_search_commits<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchCommitsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchCommits(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_search_issues<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchIssuesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchIssues(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_search_labels<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchLabelsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchLabels(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_search_repositories<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchRepositories(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_search_topics<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchTopicsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchTopics(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_search_users<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetSearchUsersResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetSearchUsers(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_teams_team_id<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetTeamsteamId(team_id), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_teams_team_id<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchTeamsteamIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchTeamsteamId(team_id), query, body)
            .await
            .unwrap()
    }
    pub async fn get_teams_team_id_discussions_discussion_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdDiscussionsdiscussionNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdDiscussionsdiscussionNumber(team_id, discussion_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_teams_team_id_discussions_discussion_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchTeamsteamIdDiscussionsdiscussionNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumber(team_id, discussion_number),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_teams_team_id_discussions_discussion_number_comments_comment_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(
                team_id,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_teams_team_id_discussions_discussion_number_comments_comment_number<T, V>(
        &self,
        team_id: String,
        discussion_number: String,
        comment_number: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumberResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdDiscussionsdiscussionNumberCommentscommentNumber(
                team_id,
                discussion_number,
                comment_number,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_teams_team_id_memberships_username<T, V>(
        &self,
        team_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdMembershipsusernameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdMembershipsusername(team_id, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_teams_team_id_memberships_username<T, V>(
        &self,
        team_id: String,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutTeamsteamIdMembershipsusernameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PutTeamsteamIdMembershipsusername(team_id, username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_teams_team_id_projects_project_id<T, V>(
        &self,
        team_id: String,
        project_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdProjectsprojectIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdProjectsprojectId(team_id, project_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_teams_team_id_repos_owner_repo<T, V>(
        &self,
        team_id: String,
        owner: String,
        repo: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdReposownerrepoResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdReposownerrepo(team_id, owner, repo),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_teams_team_id_team_sync_group_mappings<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetTeamsteamIdTeamSyncGroupMappingsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetTeamsteamIdTeamSyncGroupMappings(team_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_teams_team_id_team_sync_group_mappings<T, V>(
        &self,
        team_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchTeamsteamIdTeamSyncGroupMappingsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchTeamsteamIdTeamSyncGroupMappings(team_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_user<T, V>(&self, query: Option<&T>, body: Option<V>) -> PatchUserResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchUser(), query, body).await.unwrap()
    }
    pub async fn get_user_codespaces<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespaces(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_codespaces_secrets<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacesSecretsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespacesSecrets(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_codespaces_secrets_public_key<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacesSecretsPublicKeyResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserCodespacesSecretsPublicKey(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_codespaces_secrets_secret_name<T, V>(
        &self,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacesSecretssecretNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacesSecretssecretName(secret_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_user_codespaces_secrets_secret_name_repositories<T, V>(
        &self,
        secret_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacesSecretssecretNameRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacesSecretssecretNameRepositories(secret_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_user_codespaces_codespace_name<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacescodespaceNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacescodespaceName(codespace_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn patch_user_codespaces_codespace_name<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchUserCodespacescodespaceNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PatchUserCodespacescodespaceName(codespace_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_user_codespaces_codespace_name_machines<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserCodespacescodespaceNameMachinesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserCodespacescodespaceNameMachines(codespace_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_user_codespaces_codespace_name_start<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostUserCodespacescodespaceNameStartResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostUserCodespacescodespaceNameStart(codespace_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn post_user_codespaces_codespace_name_stop<T, V>(
        &self,
        codespace_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PostUserCodespacescodespaceNameStopResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::PostUserCodespacescodespaceNameStop(codespace_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_user_gpg_keys_gpg_key_id<T, V>(
        &self,
        gpg_key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserGpgKeysgpgKeyIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserGpgKeysgpgKeyId(gpg_key_id), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_installations<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserInstallationsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserInstallations(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_installations_installation_id_repositories<T, V>(
        &self,
        installation_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserInstallationsinstallationIdRepositoriesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserInstallationsinstallationIdRepositories(installation_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn put_user_interaction_limits<T, V>(
        &self,
        query: Option<&T>,
        body: Option<V>,
    ) -> PutUserInteractionLimitsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PutUserInteractionLimits(), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_keys_key_id<T, V>(
        &self,
        key_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserKeyskeyIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserKeyskeyId(key_id), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_memberships_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserMembershipsOrgsorgResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUserMembershipsOrgsorg(org), query, body)
            .await
            .unwrap()
    }
    pub async fn patch_user_memberships_orgs_org<T, V>(
        &self,
        org: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> PatchUserMembershipsOrgsorgResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::PatchUserMembershipsOrgsorg(org), query, body)
            .await
            .unwrap()
    }
    pub async fn get_user_migrations_migration_id<T, V>(
        &self,
        migration_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserMigrationsmigrationIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserMigrationsmigrationId(migration_id),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_user_packages_package_type_package_name<T, V>(
        &self,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserPackagespackageTypepackageNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserPackagespackageTypepackageName(package_type, package_name),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_user_packages_package_type_package_name_versions_package_version_id<T, V>(
        &self,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUserPackagespackageTypepackageNameVersionspackageVersionIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUserPackagespackageTypepackageNameVersionspackageVersionId(
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_users_username_hovercard<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernameHovercardResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(EndPoints::GetUsersusernameHovercard(username), query, body)
            .await
            .unwrap()
    }
    pub async fn get_users_username_installation<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernameInstallationResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameInstallation(username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_users_username_packages_package_type_package_name<T, V>(
        &self,
        username: String,
        package_type: String,
        package_name: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernamePackagespackageTypepackageNameResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernamePackagespackageTypepackageName(
                username,
                package_type,
                package_name,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_users_username_packages_package_type_package_name_versions_package_version_id<
        T,
        V,
    >(
        &self,
        username: String,
        package_type: String,
        package_name: String,
        package_version_id: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionIdResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernamePackagespackageTypepackageNameVersionspackageVersionId(
                username,
                package_type,
                package_name,
                package_version_id,
            ),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_users_username_settings_billing_actions<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernameSettingsBillingActionsResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingActions(username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_users_username_settings_billing_packages<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernameSettingsBillingPackagesResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingPackages(username),
            query,
            body,
        )
        .await
        .unwrap()
    }
    pub async fn get_users_username_settings_billing_shared_storage<T, V>(
        &self,
        username: String,
        query: Option<&T>,
        body: Option<V>,
    ) -> GetUsersusernameSettingsBillingSharedStorageResponse
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        self.req(
            EndPoints::GetUsersusernameSettingsBillingSharedStorage(username),
            query,
            body,
        )
        .await
        .unwrap()
    }
}
